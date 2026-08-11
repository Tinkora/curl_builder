# frozen_string_literal: true

require "optparse"
require "set"

REQUIRED_FILES = %w[
  README.md README.zh-CN.md LICENSE CHANGELOG.md MAINTAINERS.md
  CONTRIBUTING.md CONTRIBUTING.zh-CN.md SECURITY.md SECURITY.zh-CN.md
  SUPPORT.md SUPPORT.zh-CN.md CODE_OF_CONDUCT.md CODE_OF_CONDUCT.zh-CN.md
  docs/PRODUCT_SPEC.md docs/PRODUCT_SPEC.zh-CN.md
  docs/MATURITY.md docs/MATURITY.zh-CN.md .github/CODEOWNERS
].freeze
BILINGUAL_PAIRS = [
  %w[README.md README.zh-CN.md],
  %w[CONTRIBUTING.md CONTRIBUTING.zh-CN.md],
  %w[SECURITY.md SECURITY.zh-CN.md],
  %w[SUPPORT.md SUPPORT.zh-CN.md],
  %w[CODE_OF_CONDUCT.md CODE_OF_CONDUCT.zh-CN.md],
  %w[docs/PRODUCT_SPEC.md docs/PRODUCT_SPEC.zh-CN.md],
  %w[docs/MATURITY.md docs/MATURITY.zh-CN.md]
].freeze
TEXT_EXTENSIONS = %w[.html .js .json .lock .md .mjs .rb .rs .toml .yaml .yml].freeze
TEXT_FILENAMES = %w[.gitignore LICENSE].freeze
UTF8_BOM = "\xEF\xBB\xBF".b.freeze
FORBIDDEN_PUBLIC_TEXT = Regexp.new(%w[agent com mons].join, Regexp::IGNORECASE)

options = { root: Dir.pwd }
OptionParser.new do |parser|
  parser.on("--root PATH") { |path| options[:root] = path }
end.parse!

root = File.expand_path(options[:root])
errors = []
REQUIRED_FILES.each do |path|
  errors << "Missing required file: #{path}" unless File.file?(File.join(root, path))
end

BILINGUAL_PAIRS.each do |english, chinese|
  english_path = File.join(root, english)
  chinese_path = File.join(root, chinese)
  next unless File.file?(english_path) && File.file?(chinese_path)

  english_text = File.read(english_path, encoding: "UTF-8", invalid: :replace, undef: :replace)
  chinese_text = File.read(chinese_path, encoding: "UTF-8", invalid: :replace, undef: :replace)
  errors << "Missing Chinese entry link in #{english}" unless english_text.include?(File.basename(chinese))
  errors << "Missing English entry link in #{chinese}" unless chinese_text.include?(File.basename(english))
end

text_files = Dir.glob("**/*", File::FNM_DOTMATCH, base: root).select do |path|
  absolute = File.join(root, path)
  File.file?(absolute) && !path.start_with?(".git/", "target/", "pkg/", "dist/", ".playwright-cli/", "crates/curl_builder_web/node_modules/", "crates/curl_builder_web/static/", "crates/curl_builder_web/test-results/") &&
    (TEXT_EXTENSIONS.include?(File.extname(path).downcase) || TEXT_FILENAMES.include?(File.basename(path)))
end

text_files.sort.each do |path|
  bytes = File.binread(File.join(root, path))
  errors << "UTF-8 BOM is not allowed: #{path}" if bytes.start_with?(UTF8_BOM)
  content = bytes.force_encoding(Encoding::UTF_8)
  if content.valid_encoding?
    errors << "Legacy organization reference is forbidden: #{path}" if content.match?(FORBIDDEN_PUBLIC_TEXT)
  else
    errors << "Invalid UTF-8: #{path}"
  end
rescue SystemCallError => error
  errors << "Unable to read #{path}: #{error.message}"
end

if errors.empty?
  puts "Documentation checks passed (#{text_files.length} text files scanned)."
  exit 0
end

errors.each { |error| warn error }
exit 1
