const { say } = require('../pkg/ssvm_nodejs_starter_lib.js');
const { draw } = require('../pkg/ssvm_nodejs_starter_lib.js');
const buffer = require('buffer');


const http = require('http');
const url = require('url');
const hostname = '0.0.0.0';
const port = 3000;

const server = http.createServer((req, res) => {
  const u = url.parse(req.url,true);
  const queryObject = u.query;
  const action = u.pathname;
  if (action == '/draw') {
    outer_r = parseInt(queryObject['outer_r'])
    inner_r = parseInt(queryObject['inner_r'])
    dist = parseInt(queryObject['dist'])
    square_size = parseInt(queryObject['square_size'])
    r = parseInt(queryObject['r'])
    g = parseInt(queryObject['g'])
    blue = parseInt(queryObject['b'])
    var img = draw(outer_r, inner_r, dist, square_size, r, g, blue);
    res.writeHead(200, {'Content-Type': 'image/png' });
    res.end(buffer.transcode(img, 'binary', 'binary'), 'binary');
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
