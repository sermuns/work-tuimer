class WorkTuimer < Formula
  desc "Simple, keyboard-driven TUI for time-tracking"
  homepage "https://github.com/Kamyil/work-tuimer"
  url "https://github.com/Kamyil/work-tuimer/archive/refs/tags/v0.3.0.tar.gz"
  sha256 "468577cf23cab371261b2896568a539bb0bdbcdbaa0711c1653c17cb1949a6c3"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    # Test that the binary runs and responds to --version
    assert_match "work-tuimer #{version}", shell_output("#{bin}/work-tuimer --version")
    
    # Test that --help works
    assert_match "Simple, keyboard-driven TUI", shell_output("#{bin}/work-tuimer --help")
  end
end
