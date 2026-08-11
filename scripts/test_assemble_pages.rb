# frozen_string_literal: true

require "fileutils"
require "minitest/autorun"
require "open3"
require "rbconfig"
require "tmpdir"

class AssemblePagesTest < Minitest::Test
  ASSEMBLER = File.expand_path("assemble_pages.rb", __dir__)

  def test_assembles_the_application_and_wasm_package
    with_fixture do |root, wasm_package|
      result = run_assembler(root, wasm_package)

      assert result[:status].success?, result[:output]
      assert File.file?(File.join(root, "dist/index.html"))
      assert File.file?(File.join(root, "dist/assets/app.js"))
      assert File.file?(File.join(root, "dist/pkg/curl_builder_web_bg.wasm"))
    end
  end

  def test_rejects_unexpected_wasm_files
    with_fixture do |root, wasm_package|
      File.write(File.join(wasm_package, "unexpected.txt"), "no\n", encoding: "UTF-8")

      result = run_assembler(root, wasm_package)

      refute result[:status].success?
      assert_includes result[:output], "unexpected file"
    end
  end

  private

  def with_fixture
    Dir.mktmpdir("curl-builder-pages-") do |root|
      wasm_package = File.join(root, "wasm-package")
      FileUtils.mkdir_p(File.join(root, "assets"))
      FileUtils.mkdir_p(wasm_package)
      File.write(File.join(root, "index.html"), "<!doctype html>\n", encoding: "UTF-8")
      File.write(File.join(root, "assets/app.js"), "export {};\n", encoding: "UTF-8")
      File.write(File.join(wasm_package, "package.json"), "{}\n", encoding: "UTF-8")
      File.write(File.join(wasm_package, "curl_builder_web.js"), "export default {};\n", encoding: "UTF-8")
      File.binwrite(File.join(wasm_package, "curl_builder_web_bg.wasm"), "\0asm")
      yield root, wasm_package
    end
  end

  def run_assembler(root, wasm_package)
    stdout, stderr, status = Open3.capture3(
      RbConfig.ruby,
      ASSEMBLER,
      "--root",
      root,
      "--wasm-package",
      wasm_package
    )
    { output: stdout + stderr, status: status }
  end
end
