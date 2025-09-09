require 'webrick'

server = WEBrick::HTTPServer.new(Port: 3000)

server.mount_proc '/' do |req, res|
  res.body = 'Hello, world! This is a basic Ruby web server.'
end

trap('INT') { server.shutdown }

server.start
