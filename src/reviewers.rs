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
        let mut map: HashMap<String, Author> = HashMap::new();
        // FIXME: somehow dynamically generate this list. For now, it's small enough that
        // maintaining it here is not too much of a hardship.

        enum AddKind {
            New(Author),
            Alias(&'static str),
        }

        fn a(name: &str, email: &str) -> AddKind {
            AddKind::New(Author {
                name: name.into(),
                email: email.into(),
            })
        }

        fn alias(name: &'static str) -> AddKind {
            AddKind::Alias(name)
        }

        let team_people = get_team_people()?;
        for (username, person) in team_people.people {
            if let Some(email) = person.email {
                map.insert(username.to_lowercase(), Author::new(person.name, email));
            }
        }

        let mut insert = |name: &str, author: AddKind| {
            match author {
                AddKind::New(author) => if map.insert(name.into(), author).is_some() {
                    eprintln!("WARNING: Reviewer is already pulled in from rust-lang/team: {name}");
                },
                AddKind::Alias(aliased) => {
                    if let Some(author) = map.get(aliased).cloned() {
                        map.insert(name.into(), author);
                    } else {
                        eprintln!("WARNING: Trying to alias reviewer that doesn't exist: {aliased}")
                    }
                }
            }
        };

        // Additional reviewers which are not present in rust-lang/team
        insert("aaronepower", a("Erin Power", "xampprocky@gmail.com"));
        insert("abonander", a("Austin Bonander", "austin.bonander@gmail.com"));
        insert("aleksator", a("Alex Tokarev", "aleksator@gmail.com"));
        insert("alexreg", a("Alexander Regueiro", "alexreg@me.com"));
        insert("anasazi", a("Eric Reed", "ecreed@cs.washington.edu"));
        insert("apasel422", a("Andrew Paseltiner", "apaseltiner@gmail.com"));
        insert("arthurprs", a("arthurprs", "arthurprs@gmail.com"));
        insert("bblum", a("Ben Blum", "bblum@andrew.cmu.edu"));
        insert("bjz", a("Brendan Zabarauskas", "bjzaba@yahoo.com.au"));
        insert("bluss", a("Ulrik Sverdrup", "bluss@users.noreply.github.com"));
        insert("brson", a("Brian Anderson", "andersrb@gmail.com"));
        insert("bugadani", a("Dániel Buga", "bugadani@gmail.com"));
        insert("c410-f3r", a("Caio", "c410.f3r@gmail.com"));
        insert("catamorphism", a("Tim Chevalier", "chevalier@alum.wellesley.edu"));
        insert("cdirkx", a("Christiaan Dirkx", "christiaan@dirkx.email"));
        insert("chris", a("Chris Morgan", "me@chrismorgan.info"));
        insert("cldfire", a("Jarek Samic", "cldfire3@gmail.com"));
        insert("cmr", a("Corey Richardson", "corey@octayn.net"));
        insert("collin5", a("Collins Abitekaniza", "abtcolns@gmail.com"));
        insert("csmoe", a("csmoe", "csmoe@msn.com"));
        insert("dingelish", a("Yu Ding", "dingelish@gmail.com"));
        insert("dns2utf8", a("Stefan Schindler", "dns2utf8@estada.ch"));
        insert("durka", a("Alex Durka", "web@alexburka.com"));
        insert("eerden", a("Ercan Erden", "ercerden@gmail.com"));
        insert("elichai", a("Elichai Turkel", "elichai.turkel@gmail.com"));
        insert("euclio", a("Andy Russell", "arussell123@gmail.com"));
        insert("flaper87", a("Flavio Percoco", "flaper87@gmail.com"));
        insert("gereeter", a("Jonathan S", "gereeter+code@gmail.com"));
        insert("gnzlbg", a("gnzlbg", "gonzalobg88@gmail.com"));
        insert("graydon", a("Graydon Hoare", "graydon@pobox.com"));
        insert("hanna-kruppe", a("Hanna Kruppe", "hanna.kruppe@gmail.com"));
        insert("hellow554", a("Marcel Hellwig", "github@cookiesoft.de"));
        insert("ilyoan", a("ILYONG CHO", "ilyoan@gmail.com"));
        insert("jakub", a("Jakub Kądziołka", "kuba@kadziolka.net"));
        insert("jbclements", a("John Clements", "clements@racket-lang.org"));
        insert("jdm", a("Josh Matthews", "josh@joshmatthews.net"));
        insert("jethrogb", a("Jethro Beekman", "jethro@fortanix.com"));
        insert("jonas-schievink", a("Ghost", "ghost"));
        insert("jroesch", a("Jared Roesch", "roeschinc@gmail.com"));
        insert("jsgf", a("Jeremy Fitzhardinge", "jsgf@fb.com"));
        insert("kballard", a("Lily Ballard", "lily@sb.org"));
        insert("keeperofdakeys", a("Josh Driver", "keeperofdakeys@gmail.com"));
        insert("kmcallister", a("Keegan McAllister", "mcallister.keegan@gmail.com"));
        insert("kornelski", a("Kornel", "kornel@geekhood.net"));
        insert("lingman", a("LingMan", "LingMan@users.noreply.github.com"));
        insert("ljedrz", a("ljedrz", "ljedrz@gmail.com"));
        insert("lukaramu", a("lukaramu", "lukaramu@users.noreply.github.com"));
        insert("lzutao", a("Lzu Tao", "taolzu@gmail.com"));
        insert("malbarbo", a("Marco A L Barbosa", "malbarbo@gmail.com"));
        insert("marmeladema", a("marmeladema", "xademax@gmail.com"));
        insert("max-heller", a("max-heller", "max.a.heller@gmail.com"));
        insert("metajack", a("Jack Moffitt", "jack@metajack.im"));
        insert("mikerite", a("Michael Wright", "mikerite@lavabit.com"));
        insert("mikeyhew", a("Michael Hewson", "michael@michaelhewson.ca"));
        insert("mjbshaw", a("Michael Bradshaw", "mjbshaw@google.com"));
        insert("msullivan", a("Michael J. Sullivan", "sully@msully.net"));
        insert("pczarn", a("Piotr Czarnecki", "pioczarn@gmail.com"));
        insert("petrhosek", a("Petr Hosek", "phosek@gmail.com"));
        insert("pickfire", a("Ivan Tham", "pickfire@riseup.net"));
        insert("poliorcetics", a("Alexis Bourget", "alexis.bourget@gmail.com"));
        insert("raoulstrackx", a("Raoul Strackx", "raoul.strackx@fortanix.com"));
        insert("rcvalle", a("Ramon de C Valle", "rcvalle@users.noreply.github.com"));
        insert("retep998", a("Peter Atashian", "retep998@gmail.com"));
        insert("richkadel", a("Rich Kadel", "richkadel@google.com"));
        insert("sanxiyn", a("Seo Sanghyeon", "sanxiyn@gmail.com"));
        insert("seanmonstar", a("Sean McArthur", "sean@seanmonstar.com"));
        insert("stjepang", a("Stjepan Glavina", "stjepang@gmail.com"));
        insert("susurrus", a("Bryant Mairs", "bryant@mai.rs"));
        insert("swatinem", a("Arpad Borsos", "swatinem@swatinem.de"));
        insert("thestinger", a("Daniel Micay", "danielmicay@gmail.com"));
        insert("tlively", a("Thomas Lively", "tlively@google.com"));
        insert("tromey", a("Tom Tromey", "tom@tromey.com"));
        insert("vadimcn", a("Vadim Chugunov", "vadimcn@gmail.com"));
        insert("willcrichton", a("Will Crichton", "wcrichto@cs.stanford.edu"));
        insert("xfix", a("Konrad Borowski", "konrad@borowski.pw"));
        insert("yichoi", a("Young-il Choi", "duddlf.choi@samsung.com"));
        insert("y-nak", a("Yoshitomo Nakanishi", "yurayura.rounin.3@gmail.com"));
        insert("yurydelendik", a("Yury Delendik", "ydelendik@mozilla.com"));
        insert("z0w0", a("Zack Corr", "zack@z0w0.me"));
        insert("zackmdavis", a("Zack M. Davis", "code@zackmdavis.net"));

        // Aliases of r= reviewers to their current GitHub names.
        // Includes misspellings and changed names.
        insert("achrichto", alias("alexcrichton"));
        insert("acrichto", alias("alexcrichton"));
        insert("alexchrichton", alias("alexcrichton"));
        insert("alexcirchton", alias("alexcrichton"));
        insert("alexcrhiton", alias("alexcrichton"));
        insert("alexcrichto", alias("alexcrichton"));
        insert("alexcricthon", alias("alexcrichton"));
        insert("alexcricton", alias("alexcrichton"));
        insert("alexcritchton", alias("alexcrichton"));
        insert("amaneiu", alias("amanieu"));
        insert("arielb", alias("arielb1"));
        insert("bson", alias("brson"));
        insert("cgillot", alias("cjgillot"));
        insert("compiler=errors", alias("compiler-errors"));
        insert("cramert", alias("cramertj"));
        insert("cjgillo", alias("cjgillot"));
        insert("cupiver", alias("cuviper"));
        insert("ecstaticmorse", alias("ecstatic-morse"));
        insert("ekuber", alias("estebank"));
        insert("frewsxcvx", alias("frewsxcv"));
        insert("frewsxcxv", alias("frewsxcv"));
        insert("gankro", alias("gankra"));
        insert("guilliamegomez", alias("guillaumegomez"));
        insert("guilliaumegomez", alias("guillaumegomez"));
        insert("guillaumegomezp", alias("guillaumegomez"));
        insert("hi-rustin", alias("0xpoe"));
        insert("huon", alias("huonw"));
        insert("imperio", alias("guillaumegomez"));
        insert("icnr", alias("lcnr"));
        insert("jackh276", alias("jackh726"));
        insert("jakub-", alias("jakub"));
        insert("jonathandturner", alias("sophiajt"));
        insert("jubilee", alias("workingjubilee"));
        insert("jyn541", alias("jyn514"));
        insert("jyn", alias("jyn514"));
        insert("llogic", alias("llogiq"));
        insert("lncr", alias("lcnr"));
        insert("lolbinary", alias("lolbinarycat"));
        insert("manisheart", alias("manishearth"));
        insert("mark-simulacru", alias("mark-simulacrum"));
        insert("mark-simulcrum", alias("mark-simulacrum"));
        insert("marksimulacrum", alias("mark-simulacrum"));
        insert("mw", alias("michaelwoerister"));
        insert("ncr", alias("nrc"));
        insert("nick29581", alias("nrc"));
        insert("nilstrieb", alias("noratrieb"));
        insert("nmatsakis", alias("nikomatsakis"));
        insert("obi-obk", alias("oli-obk"));
        insert("oli", alias("oli-obk"));
        insert("oli-bok", alias("oli-obk"));
        insert("oli-obj", alias("oli-obk"));
        insert("ozkanonur", alias("onur-ozkan"));
        insert("petrochencov", alias("petrochenkov"));
        insert("pwalton", alias("pcwalton"));
        insert("quietmisdreqvus", alias("quietmisdreavus"));
        insert("rkruppe", alias("hanna-kruppe"));
        insert("rustin170506", alias("0xpoe"));
        insert("simulacrum", alias("mark-simulacrum"));
        insert("steveklanik", alias("steveklabnik"));
        insert("steveklbanik", alias("steveklabnik"));
        insert("wesleyweiser", alias("wesleywiser"));
        insert("withouboats", alias("withoutboats"));
        insert("yaahallo", alias("yaahc"));

        Ok(Reviewers { reviewers: map })
    }

    pub fn to_author(&self, reviewer: &str) -> Result<Option<Author>, UnknownReviewer> {
        let skip = &[
            "3",
            "burningtree",
            "docs",
            "just",
            "me",
            "new",
            "nobody",
            "rollup",
            "rustdoc",
            "rustdoc-team",
            "t-rustdoc",
            "t-rustdoc-frontend",
            "the-whole-team",
            "tinyfix",
            "update",
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
