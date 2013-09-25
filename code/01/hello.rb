10.times do
  Thread.new do
    greeting_message = "Hello?"
    puts greeting_message
  end
end
