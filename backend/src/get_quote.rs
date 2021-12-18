#[derive(Serialize)]
pub struct Quote {
    pub quote: String,
    pub source: String,
}

pub fn ret_quote(id: usize) -> Quote {
    let quotes = vec![
        Quote {quote: 
            String::from("Yes, cannabis is dangerous, but no more than other perfectly legal drugs. It's time for a rethink, and the Tory party - the funkiest, most jiving party on Earth - is where it's happening."), source: 
            String::from("Boris Johnson on cannabis")},
        Quote {quote: 
            String::from("My policy on cake is pro having it and pro eating it."), source: 
            String::from("Boris Johnson on eating cake")},
        Quote {quote: 
            String::from("There is absolutely no one, apart from yourself, who can prevent you, in the middle of the night, from sneaking down to tidy up the edges of that hunk of cheese at the back of the fridge."), source: 
            String::from("Boris Johnson on late night cheese")},
        Quote {quote: 
            String::from("My chances of being PM are about as good as the chances of finding Elvis on Mars, or my being reincarnated as an olive."), source: 
            String::from("Boris Johnson on becoming PM")},
        Quote {quote: 
            String::from("I think I was once given cocaine but I sneezed so it didn't go up my nose. In fact, it may have been icing sugar."), source: 
            String::from("Boris Johnson on hard drugs")},
        Quote {quote: 
            String::from("The only reason I wouldn't go to some parts of New York is the real risk of meeting Donald Trump."), source: 
            String::from("Boris Johnson on Donald Trump")},
        Quote {quote: 
            String::from("My speaking style was criticised by no less an authority than Arnold Schwarzenegger. It was a low moment, my friends, to have my rhetorical skills denounced by a monosyllabic Austrian cyborg."), source: 
            String::from("Boris Johnson on Austrian cyborgs")},
        Quote {quote: 
            String::from("I think it'd be disgraceful if a chap wasn't allowed to have a bit of fun in Las Vegas. The real scandal would be if you went to Vegas and you didn't misbehave in some trivial way."), source: 
            String::from("Boris Johnson on Las Vegas")},
        Quote {quote: 
            String::from("Never in my life did I think I would be congratulated by Mick Jagger for achieving anything."), source: 
            String::from("Boris Johnson on Mick Jagger")},
        Quote {quote: 
            String::from("It is absolutely ridiculous that people should choose to go around looking like letter boxes."), source: 
            String::from("Boris Johnson on the niqab")},
        Quote {quote: 
            String::from("I'm not as familiar with the works of Nicki Minaj as I probably should be"), source: 
            String::from("Boris Johnson on Nicki Minaj")},
        Quote {quote: 
            String::from("Yesterday I went, as we all must, to Peppa Pig World. Hands up if you’ve been to Peppa Pig World!"), source: 
            String::from("Boris Johnson on Peppa Pig World")},
        Quote {quote: 
            String::from("Putin? Despite looking a bit like Dobby the house-elf, he is a ruthless and manipulative tyrant."), source: 
            String::from("Boris Johnson on Vladimir Putin")},
        ];
    return Quote {
        quote: quotes[id].quote.clone(),
        source: quotes[id].source.clone(),
    }
}
