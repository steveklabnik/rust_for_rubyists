10.times do
  Thread.new do
    greeting_message = "Hello?"
    puts "#{greeting_message}" # slightly unnatural ruby to match `format!` macro in rust.
  end
end
