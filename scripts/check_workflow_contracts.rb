# frozen_string_literal: true

require "optparse"
require "yaml"

BASELINE = "d76b644c462c58ef40d2dc026271eb3584bfa5f0"
EXPECTED_CALLS = {
  ".github/workflows/quality.yml" => {
    "rust" => "Tinkora/.github/.github/workflows/reusable-rust-quality.yml@#{BASELINE}",
    "wasm" => "Tinkora/.github/.github/workflows/reusable-wasm-quality.yml@#{BASELINE}"
  },
  ".github/workflows/supply-chain.yml" => {
    "audit" => "Tinkora/.github/.github/workflows/reusable-supply-chain.yml@#{BASELINE}"
  },
  ".github/workflows/pages.yml" => {
    "deploy" => "Tinkora/.github/.github/workflows/reusable-pages.yml@#{BASELINE}"
  },
  ".github/workflows/release.yml" => {
    "evidence" => "Tinkora/.github/.github/workflows/reusable-release.yml@#{BASELINE}"
  }
}.freeze

options = { root: Dir.pwd }
OptionParser.new do |parser|
  parser.on("--root PATH") { |path| options[:root] = path }
end.parse!

root = File.expand_path(options[:root])
errors = []
workflows = {}
Dir.glob(File.join(root, ".github/workflows/*.yml")).sort.each do |path|
  relative = path.delete_prefix("#{root}/")
  begin
    workflows[relative] = YAML.safe_load_file(path, aliases: false)
  rescue Psych::Exception => error
    errors << "Invalid workflow #{relative}: #{error.message}"
  end
end

EXPECTED_CALLS.each do |path, jobs|
  workflow = workflows[path]
  if workflow.nil?
    errors << "Missing workflow: #{path}"
    next
  end
  jobs.each do |name, expected|
    actual = workflow.dig("jobs", name, "uses")
    errors << "#{path} job #{name} must use #{expected}" unless actual == expected
  end
end

workflows.each do |path, workflow|
  text = File.read(File.join(root, path), encoding: "UTF-8")
  errors << "#{path} must not use pull_request_target" if text.include?("pull_request_target")
  workflow.fetch("jobs", {}).each_value do |job|
    steps = job.fetch("steps", [])
    values = [job["uses"], *steps.map { |step| step["uses"] }].compact
    values.each do |reference|
      next if reference.start_with?("./")
      next if reference.match?(/@[0-9a-f]{40}\z/)

      errors << "#{path} contains a floating action reference: #{reference}"
    end
    steps.each do |step|
      next unless step.fetch("uses", "").start_with?("actions/upload-artifact@")

      errors << "#{path} artifact uploads must set overwrite: true" unless step.dig("with", "overwrite") == true
    end
  end
end

quality_text = File.read(File.join(root, ".github/workflows/quality.yml"), encoding: "UTF-8") rescue ""
errors << "quality.yml must require every syntax validator" unless quality_text.include?("CURL_BUILDER_REQUIRE_SYNTAX_TOOLS")
errors << "quality.yml must run the real WASM browser smoke" unless quality_text.include?("playwright-smoke: true")
errors << "quality.yml must install rustfmt for generated Rust syntax" unless quality_text.include?("rustup component add rustfmt --toolchain 1.95.0")

pages_text = File.read(File.join(root, ".github/workflows/pages.yml"), encoding: "UTF-8") rescue ""
%w[wasm-package pages-source].each do |prefix|
  marker = "#{prefix}-${{ github.run_id }}"
  errors << "pages.yml must use retry-safe #{prefix} artifacts" unless pages_text.scan(marker).length >= 1
end
errors << "pages.yml must gate assembly and deploy to main" unless pages_text.scan("github.ref == 'refs/heads/main'").length == 2

release_text = File.read(File.join(root, ".github/workflows/release.yml"), encoding: "UTF-8") rescue ""
%w[attestations: write id-token: write environment: release SBOM.spdx.json prerelease].each do |marker|
  errors << "release.yml is missing #{marker}" unless release_text.include?(marker)
end
errors << "release.yml artifact names must not depend on run_attempt" if release_text.include?("github.run_attempt")
errors << "release.yml must ignore build metadata when detecting prereleases" unless release_text.include?('version_core="${version%%+*}"')

if errors.empty?
  puts "Workflow contracts passed (organization baseline #{BASELINE})."
  exit 0
end

errors.each { |error| warn error }
exit 1
