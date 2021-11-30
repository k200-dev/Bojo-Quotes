import express from 'express'
const app = express();
const bojoQuotes = require('../quotes.json')

const port = 4000

app.get('/random', (req, res) => {
    res.header("Access-Control-Allow-Origin", "*")
    const random = Math.floor(Math.random() * bojoQuotes.quotes.length);
    res.send(bojoQuotes.quotes[random]);
})

app.listen(port, () => {
    console.log(`Server running on port: ${port}`);
})