# A sample Guardfile
# More info at https://github.com/guard/guard#readme

require 'guard/plugin'
require 'logger'

module ::Guard
  class RustTest < Plugin
    def run_all
      UI.info 'Running all cargo tests'
      system('cargo test -j 4').tap do |success|
        if success
          UI.info 'SUCCESS'
        else
          UI.error 'FAILURE'
        end
      end
    end

    def run_on_changes(paths)
      run_all
    end
  end

  class RustBench < Plugin
    def run_all
      UI.info 'Running all cargo benchmarks'
      system('cargo test -j 4 -- --bench').tap do |success|
        if success
          UI.info 'SUCCESS'
        else
          UI.error 'FAILURE'
        end
      end
    end

    def run_on_changes(paths)
      run_all
    end
  end
end

guard :rust_test do
  watch(%r{^tests/.+_test\.rs$})
  watch(%r{^src/.+\.rs$})
end

guard :rust_bench do
end
