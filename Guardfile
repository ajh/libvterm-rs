clearing :on

guard :shell do
  watch(%r/.*\.rs/) { |m| `cargo test --color always` }
end
