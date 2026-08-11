# frozen_string_literal: true

require "fileutils"
require "optparse"
require "tmpdir"

REQUIRED_WASM_FILES = %w[package.json curl_builder_web.js curl_builder_web_bg.wasm].freeze
ALLOWED_WASM_FILES = %w[
  package.json curl_builder_web.js curl_builder_web_bg.wasm curl_builder_web.d.ts
  curl_builder_web_bg.wasm.d.ts LICENSE README.md
].freeze

options = { root: File.expand_path("..", __dir__), wasm_package: nil }
OptionParser.new do |parser|
  parser.on("--root PATH") { |path| options[:root] = path }
  parser.on("--wasm-package PATH") { |path| options[:wasm_package] = path }
end.parse!

def real_file!(path, label)
  metadata = File.lstat(path)
  raise "#{label} must be a real file" unless metadata.file? && !metadata.symlink?
end

begin
  root = File.realpath(options[:root])
  source_argument = options[:wasm_package]
  raise "--wasm-package is required" if source_argument.nil? || source_argument.empty?

  source_metadata = File.lstat(source_argument)
  raise "WASM package must be a real directory" unless source_metadata.directory? && !source_metadata.symlink?
  source = File.realpath(source_argument)

  source_entries = Dir.children(source).sort
  source_entries.each do |name|
    path = File.join(source, name)
    metadata = File.lstat(path)
    raise "WASM package contains a symbolic link: #{name}" if metadata.symlink?
    raise "WASM package contains a non-file entry: #{name}" unless metadata.file?
    raise "WASM package contains an unexpected file: #{name}" unless ALLOWED_WASM_FILES.include?(name)
  end
  REQUIRED_WASM_FILES.each do |name|
    raise "WASM package is missing #{name}" unless source_entries.include?(name)
  end

  index = File.join(root, "index.html")
  assets = File.join(root, "assets")
  real_file!(index, "index.html")
  assets_metadata = File.lstat(assets)
  raise "assets must be a real directory" unless assets_metadata.directory? && !assets_metadata.symlink?
  Dir.glob(File.join(assets, "**", "*"), File::FNM_DOTMATCH).each do |path|
    next if [".", ".."].include?(File.basename(path))

    metadata = File.lstat(path)
    raise "assets contains a symbolic link" if metadata.symlink?
    raise "assets contains a special file" unless metadata.file? || metadata.directory?
  end

  output = File.join(root, "dist")
  staging = Dir.mktmpdir(".pages-build-", root)
  begin
    FileUtils.copy_file(index, File.join(staging, "index.html"))
    FileUtils.cp_r(assets, File.join(staging, "assets"))
    package_output = File.join(staging, "pkg")
    Dir.mkdir(package_output)
    source_entries.each do |name|
      FileUtils.copy_file(File.join(source, name), File.join(package_output, name))
    end
    FileUtils.rm_rf(output)
    File.rename(staging, output)
    staging = nil
    puts "Pages artifact assembled in #{output}."
  ensure
    FileUtils.remove_entry_secure(staging) if staging && File.exist?(staging)
  end
rescue OptionParser::ParseError, SystemCallError, RuntimeError => error
  warn error.message
  exit 1
end
