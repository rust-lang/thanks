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

        insert(TO-DELETE, a("Aaron Hill", "aa1ronham@gmail.com"));
        insert("aaronepower", a("Erin Power", "xampprocky@gmail.com"));
        insert(TO-DELETE, a("James Miller", "james@aatch.net"));
        insert("abonander", a("Austin Bonander", "austin.bonander@gmail.com"));
        insert("achrichto", alexcrichton.clone());
        insert("acrichto", alexcrichton.clone());
        insert(TO-DELETE, a("Alona Enraght-Moony", "code@alona.page"));
        insert(TO-DELETE, a("Aidan Hobson Sayers", "aidanhs@cantab.net"));
        insert(TO-DELETE, a("Albert Larsan", "albert.larsan@gmail.com"));
        insert("aleksator", a("Alex Tokarev", "aleksator@gmail.com"));
        insert("alexchrichton", alexcrichton.clone());
        insert("alexcirchton", alexcrichton.clone());
        insert("alexcrhiton", alexcrichton.clone());
        insert("alexcrichto", alexcrichton.clone());
        insert(TO-DELETE, alexcrichton.clone());
        insert("alexcricthon", alexcrichton.clone());
        insert("alexcricton", alexcrichton.clone());
        insert("alexcritchton", alexcrichton.clone());
        insert(TO-DELETE, a("Alex Macleod", "alex@macleod.io"));
        insert(TO-DELETE, a("Alex Butler", "alexheretic@gmail.com"));
        insert("alexreg", a("Alexander Regueiro", "alexreg@me.com"));
        insert("amaneiu", amanieu.clone());
        insert(TO-DELETE, amanieu.clone());
        insert("anasazi", a("Eric Reed", "ecreed@cs.washington.edu"));
        insert("apasel422", a("Andrew Paseltiner", "apaseltiner@gmail.com"));
        insert(TO-DELETE, a("apiraino", "apiraino@users.noreply.github.com"));
        insert(TO-DELETE, arielb1.clone());
        insert("arielb", arielb1.clone());
        insert(TO-DELETE, a("Arlo Siemsen", "arsiem@microsoft.com"));
        insert("arthurprs", a("arthurprs", "arthurprs@gmail.com"));
        insert(TO-DELETE, a("Aaron Turon", "aturon@mozilla.com"));
        insert(TO-DELETE, a("b-naber", "b_naber@gmx.de"));
        insert(TO-DELETE, a("Jan-Erik Rediger", "janerik@fnordig.de"));
        insert("bblum", a("Ben Blum", "bblum@andrew.cmu.edu"));
        insert(TO-DELETE, a("bjorn3", "bjorn3@users.noreply.github.com"));
        insert("bjz", a("Brendan Zabarauskas", "bjzaba@yahoo.com.au"));
        insert(TO-DELETE, a("Brian Koropoff", "bkoropoff@gmail.com"));
        insert("bluss", a("Ulrik Sverdrup", "bluss@users.noreply.github.com"));
        insert("brson", brson.clone());
        insert("bson", brson.clone());
        insert(TO-DELETE, a("Ben Striegel", "ben.striegel@gmail.com"));
        insert("bugadani", a("Dániel Buga", "bugadani@gmail.com"));
        insert(TO-DELETE, a("Andrew Gallant", "jamslam@gmail.com"));
        insert(TO-DELETE, a("Boxy", "supbscripter@gmail.com"));
        insert("c410-f3r", a("Caio", "c410.f3r@gmail.com"));
        insert(TO-DELETE, a("Caleb Cartwright", "caleb.cartwright@outlook.com"));
        insert(TO-DELETE, a("Camelid", "camelidcamel@gmail.com"));
        insert(TO-DELETE, a("Cameron Steffen", "cam.steffen94@gmail.com"));
        insert(TO-DELETE, a("Carl Lerche", "me@carllerche.com"));
        insert(TO-DELETE, a("Carol (Nichols || Goulding)", "carol.nichols@gmail.com"));
        insert("catamorphism", a("Tim Chevalier", "chevalier@alum.wellesley.edu"));
        insert("cdirkx", a("Christiaan Dirkx", "christiaan@dirkx.email"));
        insert(TO-DELETE, a("Mazdak Farrokhzad", "twingoow@gmail.com"));
        insert("chris", a("Chris Morgan", "me@chrismorgan.info"));
        insert(TO-DELETE, a("Chris Denton", "christophersdenton@gmail.com"));
        insert(TO-DELETE, a("Camille GILLOT", "gillot.camille@gmail.com"));
        insert("cldfire", a("Jarek Samic", "cldfire3@gmail.com"));
        insert("cmr", a("Corey Richardson", "corey@octayn.net"));
        insert("collin5", a("Collins Abitekaniza", "abtcolns@gmail.com"));
        insert(TO-DELETE, a("Michael Goulet", "michael@errs.io"));
        insert(TO-DELETE, a("Rune Tynan", "runetynan@gmail.com"));
        insert("cramert", cramertj.clone());
        insert(TO-DELETE, cramertj.clone());
        insert("csmoe", a("csmoe", "csmoe@msn.com"));
        insert(TO-DELETE, a("Josh Stone", "jistone@redhat.com"));
        insert(TO-DELETE, a("Daniel Henry-Mantilla", "daniel.henry.mantilla@gmail.com"));
        insert(TO-DELETE, a("David Wood", "david@davidtw.co"));
        insert("dingelish", a("Yu Ding", "dingelish@gmail.com"));
        insert("djc", a("Dirkjan Ochtman", "dirkjan@ochtman.nl"));
        insert(TO-DELETE, a("David Koloski", "djkoloski@gmail.com"));
        insert("dns2utf8", a("Stefan Schindler", "dns2utf8@estada.ch"));
        insert(TO-DELETE, a("Björn Steinbrink", "bsteinbr@gmail.com"));
        insert(TO-DELETE, a("dswij", "dswijj@gmail.com"));
        insert(TO-DELETE, a("David Tolnay", "dtolnay@gmail.com"));
        insert("durka", a("Alex Durka", "web@alexburka.com"));
        insert(TO-DELETE, a("Dale Wijnand", "dale.wijnand@gmail.com"));
        insert(TO-DELETE, a("dylan_DPC", "dylan.dpc@gmail.com"));
        insert(TO-DELETE, a("Eduardo Broto", "ebroto@tutanota.com"));
        insert(TO-DELETE, ecstaticmorse.clone());
        insert("ecstaticmorse", ecstaticmorse.clone());
        insert(TO-DELETE, a("Eduard-Mihai Burtescu", "edy.burt@gmail.com"));
        insert(TO-DELETE, a("Edwin Cheng", "edwin0cheng@gmail.com"));
        insert("eerden", a("Ercan Erden", "ercerden@gmail.com"));
        insert(TO-DELETE, a("Eh2406", "YeomanYaacov@gmail.com"));
        insert(TO-DELETE, a("Eric Holk", "eric@theincredibleholk.org"));
        insert(TO-DELETE, a("Eric Huss", "eric@huss.org"));
        insert("elichai", a("Elichai Turkel", "elichai.turkel@gmail.com"));
        insert(TO-DELETE, a("Ed Page", "eopage@gmail.com"));
        insert(TO-DELETE, a("Erick Tryzelaar", "erick.tryzelaar@gmail.com"));
        insert("est31", a("est31", "MTest31@outlook.com"));
        insert(TO-DELETE, a("Esteban Küber", "esteban@kuber.com.ar"));
        insert("euclio", a("Andy Russell", "arussell123@gmail.com"));
        insert(TO-DELETE, a("Deadbeef", "ent3rm4n@gmail.com"));
        insert(TO-DELETE, a("Nick Fitzgerald", "fitzgen@gmail.com"));
        insert("flaper87", a("Flavio Percoco", "flaper87@gmail.com"));
        insert(TO-DELETE, a("flip1995", "hello@philkrones.com"));
        insert(TO-DELETE, a("Florian Diebold", "flodiebold@gmail.com"));
        insert(TO-DELETE, frewsxcv.clone());
        insert("frewsxcvx", frewsxcv.clone());
        insert("frewsxcxv", frewsxcv.clone());
        insert("gankro", a("Alexis Beingessner", "a.beingessner@gmail.com"));
        insert("gereeter", a("Jonathan S", "gereeter+code@gmail.com"));
        insert(TO-DELETE, a("Takayuki Nakata", "f.seasons017@gmail.com"));
        insert("gnzlbg", a("gnzlbg", "gonzalobg88@gmail.com"));
        insert("graydon", a("Graydon Hoare", "graydon@pobox.com"));
        insert(TO-DELETE, guillaumegomez.clone());
        insert("guilliamegomez", guillaumegomez.clone());
        insert("guilliaumegomez", guillaumegomez.clone());
        insert("hanna-kruppe", hanna_kruppe.clone());
        insert("hellow554", a("Marcel Hellwig", "github@cookiesoft.de"));
        insert(TO-DELETE, a("hi-rustin", "rustin.liu@gmail.com"));
        insert(TO-DELETE, a("Hamidreza Kalbasi", "hamidrezakalbasi@protonmail.com"));
        insert("huon", huonw.clone());
        insert(TO-DELETE, huonw.clone());
        insert("ilyoan", a("ILYONG CHO", "ilyoan@gmail.com"));
        insert("imperio", guillaumegomez.clone());
        insert(TO-DELETE, a("jackh726", "jack.huey@umassmed.edu"));
        insert(TO-DELETE, a("Jakob Degen", "jakob.e.degen@gmail.com"));
        insert("jakub", jakub.clone());
        insert("jakub-", jakub.clone());
        insert(TO-DELETE, a("James Munns", "james@onevariable.com"));
        insert(TO-DELETE, a("Jorge Aparicio", "jorge@japaric.io"));
        insert(TO-DELETE, a("Jason Newcomb", "jsnewcomb@pm.me"));
        insert("jbclements", a("John Clements", "clements@racket-lang.org"));
        insert("jdm", a("Josh Matthews", "josh@joshmatthews.net"));
        insert("jethrogb", a("Jethro Beekman", "jethro@fortanix.com"));
        insert(TO-DELETE, a("Jacob Pratt", "jacob@jhpratt.dev"));
        insert(TO-DELETE, a("Yuki Okushi", "huyuumi.dev@gmail.com"));
        insert(TO-DELETE, a("Jonas Schievink", "jonasschievink@gmail.com"));
        insert("jonathandturner", a("Jonathan Turner", "jturner@mozilla.com"));
        insert(TO-DELETE, a("Josh Triplett", "josh@joshtriplett.org"));
        insert("jroesch", a("Jared Roesch", "roeschinc@gmail.com"));
        insert(TO-DELETE, a("Jeffrey Seyfried", "jeffrey.seyfried@gmail.com"));
        insert("jsgf", a("Jeremy Fitzhardinge", "jsgf@fb.com"));
        insert(TO-DELETE, a("Jacob Hoffman-Andrews", "github@hoffman-andrews.com"));
        insert("jyn", a("Joshua Nelson", "rust@jyn.dev"));
        insert(TO-DELETE, jyn514.clone());
        insert("jyn541", jyn514.clone());
        insert("kballard", a("Lily Ballard", "lily@sb.org"));
        insert("keeperofdakeys", a("Josh Driver", "keeperofdakeys@gmail.com"));
        insert(TO-DELETE, a("kennytm", "kennytm@gmail.com"));
        insert(TO-DELETE, a("Pascal Hertleif", "killercup@gmail.com"));
        insert(TO-DELETE, a("Marvin Löbel", "loebel.marvin@gmail.com"));
        insert(TO-DELETE, a("Daniel Silverstone", "dsilvers@digital-scurf.org"));
        insert("kmcallister", a("Keegan McAllister", "mcallister.keegan@gmail.com"));
        insert(TO-DELETE, a("Ashley Mannix", "ashleymannix@live.com.au"));
        insert("kornelski", a("Kornel", "kornel@geekhood.net"));
        insert(TO-DELETE, a("Bastian Kauschke", "bastian_kauschke@hotmail.de"));
        insert(TO-DELETE, a("LeSeulArtichaut", "leseulartichaut@gmail.com"));
        insert("lingman", a("LingMan", "LingMan@users.noreply.github.com"));
        insert("ljedrz", a("ljedrz", "ljedrz@gmail.com"));
        insert(TO-DELETE, llogiq.clone());
        insert("llogic", llogiq.clone());
        insert(TO-DELETE, a("Laurențiu Nicola", "lnicola@dend.ro"));
        insert(TO-DELETE, a("Lokathor", "zefria@gmail.com"));
        insert(TO-DELETE, a("Ryo Yoshida", "low.ryoshida@gmail.com"));
        insert(TO-DELETE, a("lqd", "remy.rakic+github@gmail.com"));
        insert("lukaramu", a("lukaramu", "lukaramu@users.noreply.github.com"));
        insert(TO-DELETE, a("Lukas Kalbertodt", "lukas.kalbertodt@gmail.com"));
        insert(TO-DELETE, a("Luqman Aden", "laden@csclub.uwaterloo.ca"));
        insert("lzutao", a("Lzu Tao", "taolzu@gmail.com"));
        insert("malbarbo", a("Marco A L Barbosa", "malbarbo@gmail.com"));
        insert(TO-DELETE, manishearth.clone());
        insert("manisheart", manishearth.clone());
        insert(TO-DELETE, a("Mark Mansi", "markm@cs.wisc.edu"));
        insert(TO-DELETE, simulacrum.clone());
        insert("mark-simulacru", simulacrum.clone());
        insert("mark-simulcrum", simulacrum.clone());
        insert("marksimulacrum", simulacrum.clone());
        insert("marmeladema", a("marmeladema", "xademax@gmail.com"));
        insert("mati865", a("Mateusz Mikuła", "mati865@gmail.com"));
        insert(TO-DELETE, a("Aleksey Kladov", "aleksey.kladov@gmail.com"));
        insert(TO-DELETE, a("Matthew Jasper", "mjjasper1@gmail.com"));
        insert(TO-DELETE, a("Matthias Krüger", "matthias.krueger@famsik.de"));
        insert("max-heller", a("max-heller", "max.a.heller@gmail.com"));
        insert(TO-DELETE, a("mcarton", "cartonmartin+git@gmail.com"));
        insert("metajack", a("Jack Moffitt", "jack@metajack.im"));
        insert(TO-DELETE, michaelwoerister.clone());
        insert("mikerite", a("Michael Wright", "mikerite@lavabit.com"));
        insert("mikeyhew", a("Michael Hewson", "michael@michaelhewson.ca"));
        insert("mjbshaw", a("Michael Bradshaw", "mjbshaw@google.com"));
        insert(TO-DELETE, a("Mara Bos", "m-ou.se@m-ou.se"));
        insert("msullivan", a("Michael J. Sullivan", "sully@msully.net"));
        insert(TO-DELETE, a("Scott Schafer", "schaferjscott@gmail.com"));
        insert("mw", michaelwoerister.clone());
        insert(TO-DELETE, a("Nadrieril", "nadrieril+git@gmail.com"));
        insert(TO-DELETE, a("Simonas Kazlauskas", "git@kazlauskas.me"));
        insert(TO-DELETE, a("Gary Guo", "gary@garyguo.net"));
        insert("ncr", nrc.clone());
        insert(TO-DELETE, a("Nell Shamrell-Harrington", "nellshamrell@gmail.com"));
        insert(TO-DELETE, a("Wim Looman", "git@nemo157.com"));
        insert("nick29581", nrc.clone());
        insert(TO-DELETE, a("Nikita Popov", "nikita.ppv@gmail.com"));
        insert(TO-DELETE, nikomatsakis.clone());
        insert(TO-DELETE, a("nils", "nilstrieb@gmail.com"));
        insert("nmatsakis", nikomatsakis.clone());
        insert(TO-DELETE, a("Nicholas Nethercote", "nnethercote@mozilla.com"));
        insert(TO-DELETE, a("Michael Howell", "michael@notriddle.com"));
        insert(TO-DELETE, nrc.clone());
        insert("obi-obk", oli_obk.clone());
        insert(TO-DELETE, oli_obk.clone());
        insert("oli", oli_obk.clone());
        insert(TO-DELETE, a("Oliver Middleton", "olliemail27@gmail.com"));
        insert(TO-DELETE, a("Onur Özkan", "contact@onurozkan.dev"));
        insert(TO-DELETE, pcwalton.clone());
        insert("pczarn", a("Piotr Czarnecki", "pioczarn@gmail.com"));
        insert(TO-DELETE, a("Jeremiah Peschka", "jeremiah.peschka@gmail.com"));
        insert("petrhosek", a("Petr Hosek", "phosek@gmail.com"));
        insert("petrochencov", petrochenkov.clone());
        insert(TO-DELETE, petrochenkov.clone());
        insert(TO-DELETE, a("Philipp Hansch", "dev@phansch.net"));
        insert("pickfire", a("Ivan Tham", "pickfire@riseup.net"));
        insert(TO-DELETE, a("pierwill", "rust@pierwill.com"));
        insert(TO-DELETE, a("Pietro Albini", "pietro@pietroalbini.org"));
        insert(TO-DELETE, a("Felix S Klock II", "pnkfelix@mozilla.com"));
        insert("poliorcetics", a("Alexis Bourget", "alexis.bourget@gmail.com"));
        insert("pwalton", pcwalton.clone());
        insert(TO-DELETE, quietmisdreavus.clone());
        insert("quietmisdreqvus", quietmisdreavus.clone());
        insert(TO-DELETE, a("Ralf Jung", "post@ralfj.de"));
        insert("raoulstrackx", a("Raoul Strackx", "raoul.strackx@fortanix.com"));
        insert(TO-DELETE, a("Robert Collins", "robertc@robertcollins.net"));
        insert("rcvalle", a("Ramon de C Valle", "als"));
        insert("retep998", a("Peter Atashian", "retep998@gmail.com"));
        insert("richkadel", a("Rich Kadel", "richkadel@google.com"));
        insert("rkruppe", hanna_kruppe.clone());
        insert(TO-DELETE, a("Ryan Levick", "me@ryanlevick.com"));
        insert(TO-DELETE, a("Ben Kimock", "kimockb@gmail.com"));
        insert("sanxiyn", a("Seo Sanghyeon", "sanxiyn@gmail.com"));
        insert(TO-DELETE, a("scalexm", "alexandre@scalexm.fr"));
        insert(TO-DELETE, a("Scott McMurray", "scottmcm@users.noreply.github.com"));
        insert("seanmonstar", a("Sean McArthur", "sean@seanmonstar.com"));
        insert(TO-DELETE, a("Steven Fackler", "sfackler@gmail.com"));
        insert(TO-DELETE, a("Jake Goulding", "jake.goulding@gmail.com"));
        insert(TO-DELETE, a("Simon Sapin", "simon.sapin@exyr.org"));
        insert("simulacrum", simulacrum.clone());
        insert(TO-DELETE, a("Florian Gilcher", "florian.gilcher@asquera.de"));
        insert(TO-DELETE, a("Santiago Pastorino", "spastorino@gmail.com"));
        insert(TO-DELETE, steveklabnik.clone());
        insert("steveklanik", steveklabnik.clone());
        insert("steveklbanik", steveklabnik.clone());
        insert("stjepang", a("Stjepan Glavina", "stjepang@gmail.com"));
        insert(TO-DELETE, a("Justus K", "justus.k@protonmail.com"));
        insert(TO-DELETE, a("Dan Gohman", "sunfish@mozilla.com"));
        insert("susurrus", a("Bryant Mairs", "bryant@mai.rs"));
        insert("swatinem", a("Arpad Borsos", "swatinem@swatinem.de"));
        insert(TO-DELETE, a("Takayuki Maeda", "takoyaki0316@gmail.com"));
        insert(TO-DELETE, a("The8472", "git@infinite-source.de"));
        insert("thestinger", a("Daniel Micay", "danielmicay@gmail.com"));
        insert(TO-DELETE, a("Thomas de Zeeu", "thomasdezeeuw@gmail.com"));
        insert(TO-DELETE, a("Thom Chiovoloni", "chiovolonit@gmail.com"));
        insert(TO-DELETE, a("Tim Diekmann", "tim.diekmann@3dvision.de"));
        insert(TO-DELETE, a("Tim Neumann", "timnn@google.com"));
        insert("tlively", a("Thomas Lively", "tlively@google.com"));
        insert(TO-DELETE, a("Tyler Mandry", "tmandry@gmail.com"));
        insert(TO-DELETE, a("Tomasz Miąsko", "tomasz.miasko@gmail.com"));
        insert(TO-DELETE, a("topecongiro", "seuchida@gmail.com"));
        insert("tromey", a("Tom Tromey", "tom@tromey.com"));
        insert("vadimcn", a("Vadim Chugunov", "vadimcn@gmail.com"));
        insert(TO-DELETE, a("varkor", "github@varkor.com"));
        insert(TO-DELETE, a("Lukas Wirth", "lukastw97@gmail.com"));
        insert(TO-DELETE, a("Waffle Maybe", "waffle.lapkin@gmail.com"));
        insert(TO-DELETE, a("Weihang Lo", "me@weihanglo.tw"));
        insert(TO-DELETE, a("Wesley Wiser", "wwiser@gmail.com"));
        insert("willcrichton", a("Will Crichton", "wcrichto@cs.stanford.edu"));
        insert("withouboats", withoutboats.clone());
        insert(TO-DELETE, withoutboats.clone());
        insert(TO-DELETE, a("Wodann", "wodannson@gmail.com"));
        insert(TO-DELETE, a("Jubilee Young", "workingjubilee@gmail.com"));
        insert(TO-DELETE, a("Yehuda Katz", "wycats@gmail.com"));
        insert(TO-DELETE, a("Erin Power", "xampprocky@gmail.com"));
        insert(TO-DELETE, a("Igor Matuszewski", "Xanewok@gmail.com"));
        insert("xfix", a("Konrad Borowski", "konrad@borowski.pw"));
        insert(TO-DELETE, a("xFrednet", "xFrednet@gmail.com"));
        insert("yaahallo", yaahc.clone());
        insert(TO-DELETE, yaahc.clone());
        insert("yichoi", a("Young-il Choi", "duddlf.choi@samsung.com"));
        insert("y-nak", a("Yoshitomo Nakanishi", "yurayura.rounin.3@gmail.com"));
        insert("yurydelendik", a("Yury Delendik", "ydelendik@mozilla.com"));
        insert("z0w0", a("Zack Corr", "zack@z0w0.me"));
        insert("zackmdavis", a("Zack M. Davis", "code@zackmdavis.net"));
        insert(TO-DELETE, a("John Kåre Alsaker", "john.kare.alsaker@gmail.com"));

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
