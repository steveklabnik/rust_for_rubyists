10.times.map do
  Thread.new do
    greeting_message = "Hello?"

    # This is weird in Ruby but it's closer to the println! macro
    # usage in the Rust example.
    puts "#{greeting_message}"
  end
end.each(&:join)
