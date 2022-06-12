require('dotenv').config();

const log = require("./logger").configLogger(
    process.env.log_level || "debug",
    "default"
),
    express = require("express"),
    app = express();
// bodyParser = require("body-parser"),
//   helper = require("./helper"),
// cors = require('cors');

const generateRandomString = (myLength) => {
    const chars =
        "AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz1234567890";
    const randomArray = Array.from(
        { length: myLength },
        (v, k) => chars[Math.floor(Math.random() * chars.length)]
    );

    const randomString = randomArray.join("");
    return randomString;
};

const APP_NAME = generateRandomString(10)
// app.use(bodyParser.json());
// app.use(cors());

app.get('*', (req, res, next) => {
    log.debug('Incoming Request', req.hostname, req.method, req.path);
    next();
});

app.get('/', (req, res) => {
    log.debug('Request body : ', req.body);
    res.set('Content-Type', 'text/html');
    res.send(Buffer.from("<h1>Hello from NodeJS API : " + APP_NAME + "</h1>" + "<p>Return <a href=\"/\">home</a></p>"));
    res.end()
});

app.listen(process.env.SERVER_PORT, () => {
    log.info('Generic API launched on port ' + process.env.SERVER_PORT);
});

