# frozen_string_literal: true

require "fileutils"
require "minitest/autorun"
require "open3"
require "rbconfig"
require "tmpdir"

class CheckWorkflowContractsTest < Minitest::Test
  ROOT = File.expand_path("..", __dir__)
  CHECKER = File.expand_path("check_workflow_contracts.rb", __dir__)

  def test_current_workflows_pass
    result = run_checker(ROOT)

    assert result[:status].success?, result[:output]
    assert_includes result[:output], "Workflow contracts passed"
  end

  def test_floating_reusable_reference_fails
    Dir.mktmpdir("curl-builder-workflows-") do |root|
      destination = File.join(root, ".github/workflows")
      FileUtils.mkdir_p(destination)
      FileUtils.cp_r(Dir.glob(File.join(ROOT, ".github/workflows/*.yml")), destination)
      quality_path = File.join(destination, "quality.yml")
      quality = File.read(quality_path, encoding: "UTF-8")
      quality.sub!(/@[0-9a-f]{40}/, "@main")
      File.write(quality_path, quality, encoding: "UTF-8")

      result = run_checker(root)

      refute result[:status].success?
      assert_includes result[:output], "must use"
      assert_includes result[:output], "floating action reference"
    end
  end

  def test_artifact_upload_without_overwrite_fails
    Dir.mktmpdir("curl-builder-workflows-") do |root|
      destination = File.join(root, ".github/workflows")
      FileUtils.mkdir_p(destination)
      FileUtils.cp_r(Dir.glob(File.join(ROOT, ".github/workflows/*.yml")), destination)
      pages_path = File.join(destination, "pages.yml")
      pages = File.read(pages_path, encoding: "UTF-8")
      pages.sub!("overwrite: true", "overwrite: false")
      File.write(pages_path, pages, encoding: "UTF-8")

      result = run_checker(root)

      refute result[:status].success?
      assert_includes result[:output], "must set overwrite: true"
    end
  end

  def test_artifact_names_survive_failed_job_reruns
    workflow_text = Dir.glob(File.join(ROOT, ".github/workflows/*.yml")).sort.map do |path|
      File.read(path, encoding: "UTF-8")
    end.join("\n")

    refute_includes workflow_text, "github.run_attempt"
    assert_includes workflow_text, "wasm-package-${{ github.run_id }}"
    assert_includes workflow_text, "pages-source-${{ github.run_id }}"
    assert_includes workflow_text, "release-source-${{ github.run_id }}"
    assert_includes workflow_text, "release-notes-${{ github.run_id }}"
  end

  def test_prerelease_detection_ignores_build_metadata
    release = File.read(File.join(ROOT, ".github/workflows/release.yml"), encoding: "UTF-8")

    assert_includes release, 'version_core="${version%%+*}"'
    assert_includes release, '[[ "$version_core" != *-* ]] || prerelease=true'
  end

  def test_failed_publication_removes_only_the_new_draft
    release = File.read(File.join(ROOT, ".github/workflows/release.yml"), encoding: "UTF-8")

    assert_includes release, "trap cleanup_draft ERR"
    assert_includes release, 'releases/${release_id}'
    assert_includes release, "trap - ERR"
  end

  private

  def run_checker(root)
    stdout, stderr, status = Open3.capture3(RbConfig.ruby, CHECKER, "--root", root)
    { output: stdout + stderr, status: status }
  end
end
