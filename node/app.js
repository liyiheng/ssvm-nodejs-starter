const { say } = require('../pkg/ssvm_nodejs_starter_lib.js');
const { draw } = require('../pkg/ssvm_nodejs_starter_lib.js');

const http = require('http');
const url = require('url');
const hostname = '0.0.0.0';
const port = 3000;

const server = http.createServer((req, res) => {
  const queryObject = url.parse(req.url,true).query;
  const action = request.pathname;
  if (action == '/draw') {
    outer_r = queryObject['outer_r']
    inner_r = queryObject['inner_r']
    dist = queryObject['dist']
    square_size = queryObject['square_size']
    r = queryObject['r']
    g = queryObject['g']
    b = queryObject['b']
    var img = draw(outer_r, inner_r, dist, square_size, r, g, b);
    res.writeHead(200, {'Content-Type': 'image/png' });
    res.end(img, 'binary');
    return;
  }
  if (!queryObject['name']) {
    res.end(`Please use command curl http://${hostname}:${port}/?name=MyName \n`);
  } else {
    res.end(say(queryObject['name']) + '\n');
  }
});

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});
