# A sample Guardfile
# More info at https://github.com/guard/guard#readme

require 'guard/plugin'
require 'childprocess'

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

  class JohnTestServer < Plugin
    def start
      UI.info 'Starting John Server'
      system('cargo build')
      @process = ChildProcess.build('target/john')
      @process.environment.merge!(
        "PORT" => 3100,
        "LD_LIBRARY_PATH" => "target/deps"
      )
      @process.io.inherit!
      @process.start
      @pid = @process.pid
    end

    def stop
      UI.info 'Stopping John Server'
      @process.stop if @pid && @process.alive?
    end

    def reload
      stop
      start
    end

    def run_on_changes(paths)
      reload
    end
  end
end

guard :john_test_server do
  watch(%r{^src/main.rs})
  watch(%r{^src/server.rs})
end

guard :rust_test do
  watch(%r{^tests/.+_test\.rs$})
  watch(%r{^src/lib.rs$})
  watch(%r{^src/river.rs$})
  watch(%r{^src/server.rs$})
end

guard :rust_bench do
end
