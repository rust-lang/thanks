use mailmap::Author;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

pub struct Reviewers {
    reviewers: HashMap<String, Author>,
}

impl Reviewers {
    #[rustfmt::skip]
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut map = HashMap::new();
        // FIXME: somehow dynamically generate this list. For now, it's small enough that
        // maintaining it here is not too much of a hardship.

        fn a(name: &str, email: &str) -> Author {
            Author {
                name: name.into(),
                email: email.into(),
            }
        }

        let team_people = get_team_people()?;
        for (username, person) in team_people.people {
            if let Some(email) = person.email {
                map.insert(username.to_lowercase(), a(&person.name, &email));
            }
        }


        let alexcrichton = a("Alex Crichton", "alex@alexcrichton.com");
        let amanieu = a("Amanieu d'Antras", "amanieu@gmail.com");
        let arielb1 = a("Ariel Ben-Yehuda", "ariel.byd@gmail.com");
        let brson = a("Brian Anderson", "andersrb@gmail.com");
        let cramertj = a("Taylor Cramer", "cramertj@google.com");
        let ecstaticmorse = a("Dylan MacKenzie", "ecstaticmorse@gmail.com");
        let frewsxcv = a("Corey Farwell", "coreyf@rwell.org");
        let guillaumegomez = a("Guillaume Gomez", "guillaume1.gomez@gmail.com");
        let hanna_kruppe = a("Hanna Kruppe", "hanna.kruppe@gmail.com");
        let huonw = a("Huon Wilson", "wilson.huon@gmail.com");
        let jakub = a("Jakub Kądziołka", "kuba@kadziolka.net");
        let jyn514 = a("Joshua Nelson", "jyn514@gmail.com");
        let llogiq = a("Andre Bogus", "bogusandre@gmail.com");
        let manishearth = a("Manish Goregaokar", "manishsmail@gmail.com");
        let michaelwoerister = a("Michael Woerister", "michaelwoerister@posteo.net");
        let nikomatsakis = a("Niko Matsakis", "niko@alum.mit.edu");
        let nrc = a("Nick Cameron", "ncameron@mozilla.com");
        let oli_obk = a("Oliver Scherer", "github35764891676564198441@oli-obk.de");
        let pcwalton = a("Patrick Walton", "pcwalton@mimiga.net");
        let petrochenkov = a("Vadim Petrochenkov", "vadim.petrochenkov@gmail.com");
        let quietmisdreavus = a("QuietMisdreavus", "grey@quietmisdreavus.net");
        let simulacrum = a("Mark Rousskov", "mark.simulacrum@gmail.com");
        let steveklabnik = a("Steve Klabnik", "steve@steveklabnik.com");
        let withoutboats = a("Without Boats", "boats@mozilla.com");
        let yaahc = a("Jane Lusby", "jlusby42@gmail.com");

        let mut insert = |name: &str, author| {
            if map.insert(name.into(), author).is_some() {
                println!("{name}");
            }
        };

        insert("aaronepower", a("Erin Power", "xampprocky@gmail.com"));
        insert("abonander", a("Austin Bonander", "austin.bonander@gmail.com"));
        insert("achrichto", alexcrichton.clone());
        insert("acrichto", alexcrichton.clone());
        insert("aleksator", a("Alex Tokarev", "aleksator@gmail.com"));
        insert("alexchrichton", alexcrichton.clone());
        insert("alexcirchton", alexcrichton.clone());
        insert("alexcrhiton", alexcrichton.clone());
        insert("alexcrichto", alexcrichton.clone());
        insert("alexcricthon", alexcrichton.clone());
        insert("alexcricton", alexcrichton.clone());
        insert("alexcritchton", alexcrichton.clone());
        insert("alexreg", a("Alexander Regueiro", "alexreg@me.com"));
        insert("amaneiu", amanieu.clone());
        insert("anasazi", a("Eric Reed", "ecreed@cs.washington.edu"));
        insert("apasel422", a("Andrew Paseltiner", "apaseltiner@gmail.com"));
        insert("arielb", arielb1.clone());
        insert("arthurprs", a("arthurprs", "arthurprs@gmail.com"));
        insert("bblum", a("Ben Blum", "bblum@andrew.cmu.edu"));
        insert("bjz", a("Brendan Zabarauskas", "bjzaba@yahoo.com.au"));
        insert("bluss", a("Ulrik Sverdrup", "bluss@users.noreply.github.com"));
        insert("brson", brson.clone());
        insert("bson", brson.clone());
        insert("bugadani", a("Dániel Buga", "bugadani@gmail.com"));
        insert("c410-f3r", a("Caio", "c410.f3r@gmail.com"));
        insert("catamorphism", a("Tim Chevalier", "chevalier@alum.wellesley.edu"));
        insert("cdirkx", a("Christiaan Dirkx", "christiaan@dirkx.email"));
        insert("chris", a("Chris Morgan", "me@chrismorgan.info"));
        insert("cldfire", a("Jarek Samic", "cldfire3@gmail.com"));
        insert("cmr", a("Corey Richardson", "corey@octayn.net"));
        insert("collin5", a("Collins Abitekaniza", "abtcolns@gmail.com"));
        insert("cramert", cramertj.clone());
        insert("csmoe", a("csmoe", "csmoe@msn.com"));
        insert("dingelish", a("Yu Ding", "dingelish@gmail.com"));
        insert("djc", a("Dirkjan Ochtman", "dirkjan@ochtman.nl"));
        insert("dns2utf8", a("Stefan Schindler", "dns2utf8@estada.ch"));
        insert("durka", a("Alex Durka", "web@alexburka.com"));
        insert("ecstaticmorse", ecstaticmorse.clone());
        insert("eerden", a("Ercan Erden", "ercerden@gmail.com"));
        insert("elichai", a("Elichai Turkel", "elichai.turkel@gmail.com"));
        insert("est31", a("est31", "MTest31@outlook.com"));
        insert("euclio", a("Andy Russell", "arussell123@gmail.com"));
        insert("flaper87", a("Flavio Percoco", "flaper87@gmail.com"));
        insert("frewsxcvx", frewsxcv.clone());
        insert("frewsxcxv", frewsxcv.clone());
        insert("gankro", a("Alexis Beingessner", "a.beingessner@gmail.com"));
        insert("gereeter", a("Jonathan S", "gereeter+code@gmail.com"));
        insert("gnzlbg", a("gnzlbg", "gonzalobg88@gmail.com"));
        insert("graydon", a("Graydon Hoare", "graydon@pobox.com"));
        insert("guilliamegomez", guillaumegomez.clone());
        insert("guilliaumegomez", guillaumegomez.clone());
        insert("hanna-kruppe", hanna_kruppe.clone());
        insert("hellow554", a("Marcel Hellwig", "github@cookiesoft.de"));
        insert("huon", huonw.clone());
        insert("ilyoan", a("ILYONG CHO", "ilyoan@gmail.com"));
        insert("imperio", guillaumegomez.clone());
        insert("jakub", jakub.clone());
        insert("jakub-", jakub.clone());
        insert("jbclements", a("John Clements", "clements@racket-lang.org"));
        insert("jdm", a("Josh Matthews", "josh@joshmatthews.net"));
        insert("jethrogb", a("Jethro Beekman", "jethro@fortanix.com"));
        insert("jonathandturner", a("Jonathan Turner", "jturner@mozilla.com"));
        insert("jroesch", a("Jared Roesch", "roeschinc@gmail.com"));
        insert("jsgf", a("Jeremy Fitzhardinge", "jsgf@fb.com"));
        insert("jyn", a("Joshua Nelson", "rust@jyn.dev"));
        insert("jyn541", jyn514.clone());
        insert("kballard", a("Lily Ballard", "lily@sb.org"));
        insert("keeperofdakeys", a("Josh Driver", "keeperofdakeys@gmail.com"));
        insert("kmcallister", a("Keegan McAllister", "mcallister.keegan@gmail.com"));
        insert("kornelski", a("Kornel", "kornel@geekhood.net"));
        insert("lingman", a("LingMan", "LingMan@users.noreply.github.com"));
        insert("ljedrz", a("ljedrz", "ljedrz@gmail.com"));
        insert("llogic", llogiq.clone());
        insert("lukaramu", a("lukaramu", "lukaramu@users.noreply.github.com"));
        insert("lzutao", a("Lzu Tao", "taolzu@gmail.com"));
        insert("malbarbo", a("Marco A L Barbosa", "malbarbo@gmail.com"));
        insert("manisheart", manishearth.clone());
        insert("mark-simulacru", simulacrum.clone());
        insert("mark-simulcrum", simulacrum.clone());
        insert("marksimulacrum", simulacrum.clone());
        insert("marmeladema", a("marmeladema", "xademax@gmail.com"));
        insert("mati865", a("Mateusz Mikuła", "mati865@gmail.com"));
        insert("max-heller", a("max-heller", "max.a.heller@gmail.com"));
        insert("metajack", a("Jack Moffitt", "jack@metajack.im"));
        insert("mikerite", a("Michael Wright", "mikerite@lavabit.com"));
        insert("mikeyhew", a("Michael Hewson", "michael@michaelhewson.ca"));
        insert("mjbshaw", a("Michael Bradshaw", "mjbshaw@google.com"));
        insert("msullivan", a("Michael J. Sullivan", "sully@msully.net"));
        insert("mw", michaelwoerister.clone());
        insert("ncr", nrc.clone());
        insert("nick29581", nrc.clone());
        insert("nmatsakis", nikomatsakis.clone());
        insert("obi-obk", oli_obk.clone());
        insert("oli", oli_obk.clone());
        insert("pczarn", a("Piotr Czarnecki", "pioczarn@gmail.com"));
        insert("petrhosek", a("Petr Hosek", "phosek@gmail.com"));
        insert("petrochencov", petrochenkov.clone());
        insert("pickfire", a("Ivan Tham", "pickfire@riseup.net"));
        insert("poliorcetics", a("Alexis Bourget", "alexis.bourget@gmail.com"));
        insert("pwalton", pcwalton.clone());
        insert("quietmisdreqvus", quietmisdreavus.clone());
        insert("raoulstrackx", a("Raoul Strackx", "raoul.strackx@fortanix.com"));
        insert("rcvalle", a("Ramon de C Valle", "als"));
        insert("retep998", a("Peter Atashian", "retep998@gmail.com"));
        insert("richkadel", a("Rich Kadel", "richkadel@google.com"));
        insert("rkruppe", hanna_kruppe.clone());
        insert("sanxiyn", a("Seo Sanghyeon", "sanxiyn@gmail.com"));
        insert("seanmonstar", a("Sean McArthur", "sean@seanmonstar.com"));
        insert("simulacrum", simulacrum.clone());
        insert("steveklanik", steveklabnik.clone());
        insert("steveklbanik", steveklabnik.clone());
        insert("stjepang", a("Stjepan Glavina", "stjepang@gmail.com"));
        insert("susurrus", a("Bryant Mairs", "bryant@mai.rs"));
        insert("swatinem", a("Arpad Borsos", "swatinem@swatinem.de"));
        insert("thestinger", a("Daniel Micay", "danielmicay@gmail.com"));
        insert("tlively", a("Thomas Lively", "tlively@google.com"));
        insert("tromey", a("Tom Tromey", "tom@tromey.com"));
        insert("vadimcn", a("Vadim Chugunov", "vadimcn@gmail.com"));
        insert("willcrichton", a("Will Crichton", "wcrichto@cs.stanford.edu"));
        insert("withouboats", withoutboats.clone());
        insert("xfix", a("Konrad Borowski", "konrad@borowski.pw"));
        insert("yaahallo", yaahc.clone());
        insert("yichoi", a("Young-il Choi", "duddlf.choi@samsung.com"));
        insert("y-nak", a("Yoshitomo Nakanishi", "yurayura.rounin.3@gmail.com"));
        insert("yurydelendik", a("Yury Delendik", "ydelendik@mozilla.com"));
        insert("z0w0", a("Zack Corr", "zack@z0w0.me"));
        insert("zackmdavis", a("Zack M. Davis", "code@zackmdavis.net"));

        Ok(Reviewers { reviewers: map })
    }

    pub fn to_author(&self, reviewer: &str) -> Result<Option<Author>, UnknownReviewer> {
        let skip = &[
            "me",
            "just",
            "new",
            "rollup",
            "burningtree",
            "tinyfix",
            "update",
            "3",
            "the-whole-team",
            "nobody",
            "docs",
        ];
        let reviewer = reviewer.to_lowercase();
        if skip.contains(&reviewer.as_str()) {
            return Ok(None);
        }
        if let Some(v) = self.reviewers.get(reviewer.as_str()).cloned() {
            return Ok(Some(v));
        }
        Err(UnknownReviewer)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct UnknownReviewer;

impl std::error::Error for UnknownReviewer {}

impl fmt::Display for UnknownReviewer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown reviewer")
    }
}

#[derive(Debug, Deserialize)]

pub struct TeamPeople {
    pub people: HashMap<String, TeamPerson>,
}

#[derive(Debug, Deserialize)]
pub struct TeamPerson {
    pub name: String,
    pub email: Option<String>,
}

pub fn get_team_people() -> Result<TeamPeople, Box<dyn Error>> {
    Ok(
        ureq::get("https://team-api.infra.rust-lang.org/v1/people.json")
            .call()?
            .into_json::<TeamPeople>()?,
    )
}
