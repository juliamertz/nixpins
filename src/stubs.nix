let
  # passthrough args for enum deserialization with tvix
  enum = name: attrs: { ${name} = attrs; };
  fetchFromGitHub = enum "Github";
  fetchFromGitLab = enum "Gitlab";
in
