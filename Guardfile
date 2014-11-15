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
    def start_a_service(port, introduce_to)
      UI.info "Starting John Server on port #{port}"
      system('cargo build')

      @process ||= {}
      @pid ||= {}

      @process[port] = ChildProcess.build('target/john')
      @process[port].environment.merge!(
        "RAFT_HOST" => "localhost:#{port}",
        "PORT" => port,
        "LD_LIBRARY_PATH" => "target/deps"
      )
      @process[port].environment.merge!("RAFT_INTRODUCE" => "localhost:#{introduce_to}") unless introduce_to == port

      @process[port].io.inherit!
      @process[port].start
      @pid[port] = @process[port].pid
    end

    def start
      [3100, 3200, 3300].each do |port|
        start_a_service(port, 3100)
      end
    end

    def stop
      UI.info 'Stopping John Server'
      @process.each do |port, process|
        process.stop if @pid[port] && process.alive?
      end
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
  watch(%r{^src/raft.rs$})
end

guard :rust_bench do
end
