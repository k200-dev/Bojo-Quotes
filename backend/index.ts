import express from 'express'
const app = express();
const bojoQuotes = require('../quotes.json')

app.get('/api/random', (req, res) => {
    res.header("Access-Control-Allow-Origin", "*")
    const random = Math.floor(Math.random() * bojoQuotes.quotes.length);
    res.send(bojoQuotes.quotes[random]);
})

app.listen(4000, () => {
    console.log('Server running on port 4000');
})