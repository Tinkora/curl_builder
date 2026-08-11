# frozen_string_literal: true

require "English"
require "json"
require "optparse"

SEMVER = /\A(?:0|[1-9]\d*)\.(?:0|[1-9]\d*)\.(?:0|[1-9]\d*)(?:-(?:(?:0|[1-9]\d*)|(?:\d*[A-Za-z-][0-9A-Za-z-]*))(?:\.(?:(?:0|[1-9]\d*)|(?:\d*[A-Za-z-][0-9A-Za-z-]*)))*)?(?:\+[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?\z/

options = {}
OptionParser.new do |parser|
  parser.on("--tag TAG") { |value| options[:tag] = value }
  parser.on("--notes PATH") { |value| options[:notes] = value }
end.parse!

tag = options.fetch(:tag)
notes_path = options.fetch(:notes)
abort("tag must be v followed by full SemVer") unless tag.start_with?("v") && tag.delete_prefix("v").match?(SEMVER)
version = tag.delete_prefix("v")

metadata_output = `cargo metadata --format-version 1 --no-deps --locked`
abort("cargo metadata failed") unless $CHILD_STATUS.success?
metadata = JSON.parse(metadata_output)
members = metadata.fetch("workspace_members")
packages = metadata.fetch("packages").select { |package| members.include?(package.fetch("id")) }
abort("workspace has no packages") if packages.empty?
abort("workspace package versions do not match #{version}") unless packages.all? do |package|
  package.fetch("version") == version
end

changelog = File.read("CHANGELOG.md", encoding: "UTF-8")
header = /^## \[#{Regexp.escape(version)}\] - \d{4}-\d{2}-\d{2}$/
match = changelog.match(header)
abort("CHANGELOG.md has no #{version} release section") unless match
boundary = changelog.match(/^## /, match.end(0))
links = changelog.match(/^\[[^\]]+\]:\s+\S+$/, match.end(0))
finish = [boundary&.begin(0), links&.begin(0)].compact.min || changelog.length
notes = changelog[match.end(0)...finish].strip
abort("CHANGELOG.md #{version} release section is empty") if notes.empty?
File.write(notes_path, "#{notes}\n", encoding: "UTF-8")
