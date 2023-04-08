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

        insert("aaron1011", a("Aaron Hill", "aa1ronham@gmail.com"));
        insert("aaronepower", a("Erin Power", "xampprocky@gmail.com"));
        insert("aatch", a("James Miller", "james@aatch.net"));
        insert("abonander", a("Austin Bonander", "austin.bonander@gmail.com"));
        insert("achrichto", alexcrichton.clone());
        insert("acrichto", alexcrichton.clone());
        insert("adotinthevoid", a("Alona Enraght-Moony", "code@alona.page"));
        insert("aidanhs", a("Aidan Hobson Sayers", "aidanhs@cantab.net"));
        insert("albertlarsan68", a("Albert Larsan", "albert.larsan@gmail.com"));
        insert("aleksator", a("Alex Tokarev", "aleksator@gmail.com"));
        insert("alexchrichton", alexcrichton.clone());
        insert("alexcirchton", alexcrichton.clone());
        insert("alexcrhiton", alexcrichton.clone());
        insert("alexcrichto", alexcrichton.clone());
        insert("alexcrichton", alexcrichton.clone());
        insert("alexcricthon", alexcrichton.clone());
        insert("alexcricton", alexcrichton.clone());
        insert("alexcritchton", alexcrichton.clone());
        insert("alexendoo", a("Alex Macleod", "alex@macleod.io"));
        insert("alexheretic", a("Alex Butler", "alexheretic@gmail.com"));
        insert("alexreg", a("Alexander Regueiro", "alexreg@me.com"));
        insert("amaneiu", amanieu.clone());
        insert("amanieu", amanieu.clone());
        insert("anasazi", a("Eric Reed", "ecreed@cs.washington.edu"));
        insert("apasel422", a("Andrew Paseltiner", "apaseltiner@gmail.com"));
        insert("apiraino", a("apiraino", "apiraino@users.noreply.github.com"));
        insert("arielb1", arielb1.clone());
        insert("arielb", arielb1.clone());
        insert("arlosi", a("Arlo Siemsen", "arsiem@microsoft.com"));
        insert("arthurprs", a("arthurprs", "arthurprs@gmail.com"));
        insert("aturon", a("Aaron Turon", "aturon@mozilla.com"));
        insert("b-naber", a("b-naber", "b_naber@gmx.de"));
        insert("badboy", a("Jan-Erik Rediger", "janerik@fnordig.de"));
        insert("bblum", a("Ben Blum", "bblum@andrew.cmu.edu"));
        insert("bjorn3", a("bjorn3", "bjorn3@users.noreply.github.com"));
        insert("bjz", a("Brendan Zabarauskas", "bjzaba@yahoo.com.au"));
        insert("bkoropoff", a("Brian Koropoff", "bkoropoff@gmail.com"));
        insert("bluss", a("Ulrik Sverdrup", "bluss@users.noreply.github.com"));
        insert("brson", brson.clone());
        insert("bson", brson.clone());
        insert("bstrie", a("Ben Striegel", "ben.striegel@gmail.com"));
        insert("bugadani", a("Dániel Buga", "bugadani@gmail.com"));
        insert("burntsushi", a("Andrew Gallant", "jamslam@gmail.com"));
        insert("boxyuwu", a("Boxy", "supbscripter@gmail.com"));
        insert("c410-f3r", a("Caio", "c410.f3r@gmail.com"));
        insert("calebcartwright", a("Caleb Cartwright", "caleb.cartwright@outlook.com"));
        insert("camelid", a("Camelid", "camelidcamel@gmail.com"));
        insert("camsteffen", a("Cameron Steffen", "cam.steffen94@gmail.com"));
        insert("carllerche", a("Carl Lerche", "me@carllerche.com"));
        insert("carols10cents", a("Carol (Nichols || Goulding)", "carol.nichols@gmail.com"));
        insert("catamorphism", a("Tim Chevalier", "chevalier@alum.wellesley.edu"));
        insert("cdirkx", a("Christiaan Dirkx", "christiaan@dirkx.email"));
        insert("centril", a("Mazdak Farrokhzad", "twingoow@gmail.com"));
        insert("chris", a("Chris Morgan", "me@chrismorgan.info"));
        insert("chrisdenton", a("Chris Denton", "christophersdenton@gmail.com"));
        insert("cjgillot", a("Camille GILLOT", "gillot.camille@gmail.com"));
        insert("cldfire", a("Jarek Samic", "cldfire3@gmail.com"));
        insert("cmr", a("Corey Richardson", "corey@octayn.net"));
        insert("collin5", a("Collins Abitekaniza", "abtcolns@gmail.com"));
        insert("compiler-errors", a("Michael Goulet", "michael@errs.io"));
        insert("craftspider", a("Rune Tynan", "runetynan@gmail.com"));
        insert("cramert", cramertj.clone());
        insert("cramertj", cramertj.clone());
        insert("csmoe", a("csmoe", "csmoe@msn.com"));
        insert("cuviper", a("Josh Stone", "jistone@redhat.com"));
        insert("danielhenrymantilla", a("Daniel Henry-Mantilla", "daniel.henry.mantilla@gmail.com"));
        insert("davidtwco", a("David Wood", "david@davidtw.co"));
        insert("dingelish", a("Yu Ding", "dingelish@gmail.com"));
        insert("djc", a("Dirkjan Ochtman", "dirkjan@ochtman.nl"));
        insert("djkoloski", a("David Koloski", "djkoloski@gmail.com"));
        insert("dns2utf8", a("Stefan Schindler", "dns2utf8@estada.ch"));
        insert("dotdash", a("Björn Steinbrink", "bsteinbr@gmail.com"));
        insert("dswij", a("dswij", "dswijj@gmail.com"));
        insert("dtolnay", a("David Tolnay", "dtolnay@gmail.com"));
        insert("durka", a("Alex Durka", "web@alexburka.com"));
        insert("dwijnand", a("Dale Wijnand", "dale.wijnand@gmail.com"));
        insert("dylan-dpc", a("dylan_DPC", "dylan.dpc@gmail.com"));
        insert("ebroto", a("Eduardo Broto", "ebroto@tutanota.com"));
        insert("ecstatic-morse", ecstaticmorse.clone());
        insert("ecstaticmorse", ecstaticmorse.clone());
        insert("eddyb", a("Eduard-Mihai Burtescu", "edy.burt@gmail.com"));
        insert("edwin0cheng", a("Edwin Cheng", "edwin0cheng@gmail.com"));
        insert("eerden", a("Ercan Erden", "ercerden@gmail.com"));
        insert("eh2406", a("Eh2406", "YeomanYaacov@gmail.com"));
        insert("eholk", a("Eric Holk", "eric@theincredibleholk.org"));
        insert("ehuss", a("Eric Huss", "eric@huss.org"));
        insert("elichai", a("Elichai Turkel", "elichai.turkel@gmail.com"));
        insert("epage", a("Ed Page", "eopage@gmail.com"));
        insert("erickt", a("Erick Tryzelaar", "erick.tryzelaar@gmail.com"));
        insert("est31", a("est31", "MTest31@outlook.com"));
        insert("estebank", a("Esteban Küber", "esteban@kuber.com.ar"));
        insert("euclio", a("Andy Russell", "arussell123@gmail.com"));
        insert("fee1-dead", a("Deadbeef", "ent3rm4n@gmail.com"));
        insert("fitzgen", a("Nick Fitzgerald", "fitzgen@gmail.com"));
        insert("flaper87", a("Flavio Percoco", "flaper87@gmail.com"));
        insert("flip1995", a("flip1995", "hello@philkrones.com"));
        insert("flodiebold", a("Florian Diebold", "flodiebold@gmail.com"));
        insert("frewsxcv", frewsxcv.clone());
        insert("frewsxcvx", frewsxcv.clone());
        insert("frewsxcxv", frewsxcv.clone());
        insert("gankro", a("Alexis Beingessner", "a.beingessner@gmail.com"));
        insert("gereeter", a("Jonathan S", "gereeter+code@gmail.com"));
        insert("giraffate", a("Takayuki Nakata", "f.seasons017@gmail.com"));
        insert("gnzlbg", a("gnzlbg", "gonzalobg88@gmail.com"));
        insert("graydon", a("Graydon Hoare", "graydon@pobox.com"));
        insert("guillaumegomez", guillaumegomez.clone());
        insert("guilliamegomez", guillaumegomez.clone());
        insert("guilliaumegomez", guillaumegomez.clone());
        insert("hanna-kruppe", hanna_kruppe.clone());
        insert("hellow554", a("Marcel Hellwig", "github@cookiesoft.de"));
        insert("hi-rustin", a("hi-rustin", "rustin.liu@gmail.com"));
        insert("hkalbasi", a("Hamidreza Kalbasi", "hamidrezakalbasi@protonmail.com"));
        insert("huon", huonw.clone());
        insert("huonw", huonw.clone());
        insert("ilyoan", a("ILYONG CHO", "ilyoan@gmail.com"));
        insert("imperio", guillaumegomez.clone());
        insert("jackh726", a("jackh726", "jack.huey@umassmed.edu"));
        insert("jakobdegen", a("Jakob Degen", "jakob.e.degen@gmail.com"));
        insert("jakub", jakub.clone());
        insert("jakub-", jakub.clone());
        insert("jamesmunns", a("James Munns", "james@onevariable.com"));
        insert("japaric", a("Jorge Aparicio", "jorge@japaric.io"));
        insert("jarcho", a("Jason Newcomb", "jsnewcomb@pm.me"));
        insert("jbclements", a("John Clements", "clements@racket-lang.org"));
        insert("jdm", a("Josh Matthews", "josh@joshmatthews.net"));
        insert("jethrogb", a("Jethro Beekman", "jethro@fortanix.com"));
        insert("jhpratt", a("Jacob Pratt", "jacob@jhpratt.dev"));
        insert("johntitor", a("Yuki Okushi", "huyuumi.dev@gmail.com"));
        insert("jonas-schievink", a("Jonas Schievink", "jonasschievink@gmail.com"));
        insert("jonathandturner", a("Jonathan Turner", "jturner@mozilla.com"));
        insert("joshtriplett", a("Josh Triplett", "josh@joshtriplett.org"));
        insert("jroesch", a("Jared Roesch", "roeschinc@gmail.com"));
        insert("jseyfried", a("Jeffrey Seyfried", "jeffrey.seyfried@gmail.com"));
        insert("jsgf", a("Jeremy Fitzhardinge", "jsgf@fb.com"));
        insert("jsha", a("Jacob Hoffman-Andrews", "github@hoffman-andrews.com"));
        insert("jyn", a("Joshua Nelson", "rust@jyn.dev"));
        insert("jyn514", jyn514.clone());
        insert("jyn541", jyn514.clone());
        insert("kballard", a("Lily Ballard", "lily@sb.org"));
        insert("keeperofdakeys", a("Josh Driver", "keeperofdakeys@gmail.com"));
        insert("kennytm", a("kennytm", "kennytm@gmail.com"));
        insert("killercup", a("Pascal Hertleif", "killercup@gmail.com"));
        insert("kimundi", a("Marvin Löbel", "loebel.marvin@gmail.com"));
        insert("kinnison", a("Daniel Silverstone", "dsilvers@digital-scurf.org"));
        insert("kmcallister", a("Keegan McAllister", "mcallister.keegan@gmail.com"));
        insert("kodraus", a("Ashley Mannix", "ashleymannix@live.com.au"));
        insert("kornelski", a("Kornel", "kornel@geekhood.net"));
        insert("lcnr", a("Bastian Kauschke", "bastian_kauschke@hotmail.de"));
        insert("leseulartichaut", a("LeSeulArtichaut", "leseulartichaut@gmail.com"));
        insert("lingman", a("LingMan", "LingMan@users.noreply.github.com"));
        insert("ljedrz", a("ljedrz", "ljedrz@gmail.com"));
        insert("llogiq", llogiq.clone());
        insert("llogic", llogiq.clone());
        insert("lnicola", a("Laurențiu Nicola", "lnicola@dend.ro"));
        insert("lokathor", a("Lokathor", "zefria@gmail.com"));
        insert("lowr", a("Ryo Yoshida", "low.ryoshida@gmail.com"));
        insert("lqd", a("lqd", "remy.rakic+github@gmail.com"));
        insert("lukaramu", a("lukaramu", "lukaramu@users.noreply.github.com"));
        insert("lukaskalbertodt", a("Lukas Kalbertodt", "lukas.kalbertodt@gmail.com"));
        insert("luqmana", a("Luqman Aden", "laden@csclub.uwaterloo.ca"));
        insert("lzutao", a("Lzu Tao", "taolzu@gmail.com"));
        insert("malbarbo", a("Marco A L Barbosa", "malbarbo@gmail.com"));
        insert("manishearth", manishearth.clone());
        insert("manisheart", manishearth.clone());
        insert("mark-i-m", a("Mark Mansi", "markm@cs.wisc.edu"));
        insert("mark-simulacrum", simulacrum.clone());
        insert("mark-simulacru", simulacrum.clone());
        insert("mark-simulcrum", simulacrum.clone());
        insert("marksimulacrum", simulacrum.clone());
        insert("marmeladema", a("marmeladema", "xademax@gmail.com"));
        insert("mati865", a("Mateusz Mikuła", "mati865@gmail.com"));
        insert("matklad", a("Aleksey Kladov", "aleksey.kladov@gmail.com"));
        insert("matthewjasper", a("Matthew Jasper", "mjjasper1@gmail.com"));
        insert("matthiaskrgr", a("Matthias Krüger", "matthias.krueger@famsik.de"));
        insert("max-heller", a("max-heller", "max.a.heller@gmail.com"));
        insert("mcarton", a("mcarton", "cartonmartin+git@gmail.com"));
        insert("metajack", a("Jack Moffitt", "jack@metajack.im"));
        insert("michaelwoerister", michaelwoerister.clone());
        insert("mikerite", a("Michael Wright", "mikerite@lavabit.com"));
        insert("mikeyhew", a("Michael Hewson", "michael@michaelhewson.ca"));
        insert("mjbshaw", a("Michael Bradshaw", "mjbshaw@google.com"));
        insert("m-ou-se", a("Mara Bos", "m-ou.se@m-ou.se"));
        insert("msullivan", a("Michael J. Sullivan", "sully@msully.net"));
        insert("muscraft", a("Scott Schafer", "schaferjscott@gmail.com"));
        insert("mw", michaelwoerister.clone());
        insert("nadrieril", a("Nadrieril", "nadrieril+git@gmail.com"));
        insert("nagisa", a("Simonas Kazlauskas", "git@kazlauskas.me"));
        insert("nbdd0121", a("Gary Guo", "gary@garyguo.net"));
        insert("ncr", nrc.clone());
        insert("nellshamrell", a("Nell Shamrell-Harrington", "nellshamrell@gmail.com"));
        insert("nemo157", a("Wim Looman", "git@nemo157.com"));
        insert("nick29581", nrc.clone());
        insert("nikic", a("Nikita Popov", "nikita.ppv@gmail.com"));
        insert("nikomatsakis", nikomatsakis.clone());
        insert("nilstrieb", a("nils", "nilstrieb@gmail.com"));
        insert("nmatsakis", nikomatsakis.clone());
        insert("nnethercote", a("Nicholas Nethercote", "nnethercote@mozilla.com"));
        insert("notriddle", a("Michael Howell", "michael@notriddle.com"));
        insert("nrc", nrc.clone());
        insert("obi-obk", oli_obk.clone());
        insert("oli-obk", oli_obk.clone());
        insert("oli", oli_obk.clone());
        insert("ollie27", a("Oliver Middleton", "olliemail27@gmail.com"));
        insert("ozkanonur", a("Onur Özkan", "contact@onurozkan.dev"));
        insert("pcwalton", pcwalton.clone());
        insert("pczarn", a("Piotr Czarnecki", "pioczarn@gmail.com"));
        insert("peschkaj", a("Jeremiah Peschka", "jeremiah.peschka@gmail.com"));
        insert("petrhosek", a("Petr Hosek", "phosek@gmail.com"));
        insert("petrochencov", petrochenkov.clone());
        insert("petrochenkov", petrochenkov.clone());
        insert("phansch", a("Philipp Hansch", "dev@phansch.net"));
        insert("pickfire", a("Ivan Tham", "pickfire@riseup.net"));
        insert("pierwill", a("pierwill", "rust@pierwill.com"));
        insert("pietroalbini", a("Pietro Albini", "pietro@pietroalbini.org"));
        insert("pnkfelix", a("Felix S Klock II", "pnkfelix@mozilla.com"));
        insert("poliorcetics", a("Alexis Bourget", "alexis.bourget@gmail.com"));
        insert("pwalton", pcwalton.clone());
        insert("quietmisdreavus", quietmisdreavus.clone());
        insert("quietmisdreqvus", quietmisdreavus.clone());
        insert("ralfjung", a("Ralf Jung", "post@ralfj.de"));
        insert("raoulstrackx", a("Raoul Strackx", "raoul.strackx@fortanix.com"));
        insert("rbtcollins", a("Robert Collins", "robertc@robertcollins.net"));
        insert("rcvalle", a("Ramon de C Valle", "als"));
        insert("retep998", a("Peter Atashian", "retep998@gmail.com"));
        insert("richkadel", a("Rich Kadel", "richkadel@google.com"));
        insert("rkruppe", hanna_kruppe.clone());
        insert("rylev", a("Ryan Levick", "me@ryanlevick.com"));
        insert("saethlin", a("Ben Kimock", "kimockb@gmail.com"));
        insert("sanxiyn", a("Seo Sanghyeon", "sanxiyn@gmail.com"));
        insert("scalexm", a("scalexm", "alexandre@scalexm.fr"));
        insert("scottmcm", a("Scott McMurray", "scottmcm@users.noreply.github.com"));
        insert("seanmonstar", a("Sean McArthur", "sean@seanmonstar.com"));
        insert("sfackler", a("Steven Fackler", "sfackler@gmail.com"));
        insert("shepmaster", a("Jake Goulding", "jake.goulding@gmail.com"));
        insert("simonsapin", a("Simon Sapin", "simon.sapin@exyr.org"));
        insert("simulacrum", simulacrum.clone());
        insert("skade", a("Florian Gilcher", "florian.gilcher@asquera.de"));
        insert("spastorino", a("Santiago Pastorino", "spastorino@gmail.com"));
        insert("steveklabnik", steveklabnik.clone());
        insert("steveklanik", steveklabnik.clone());
        insert("steveklbanik", steveklabnik.clone());
        insert("stjepang", a("Stjepan Glavina", "stjepang@gmail.com"));
        insert("stupremee", a("Justus K", "justus.k@protonmail.com"));
        insert("sunfishcode", a("Dan Gohman", "sunfish@mozilla.com"));
        insert("susurrus", a("Bryant Mairs", "bryant@mai.rs"));
        insert("swatinem", a("Arpad Borsos", "swatinem@swatinem.de"));
        insert("tako8ki", a("Takayuki Maeda", "takoyaki0316@gmail.com"));
        insert("the8472", a("The8472", "git@infinite-source.de"));
        insert("thestinger", a("Daniel Micay", "danielmicay@gmail.com"));
        insert("thomasdezeeuw", a("Thomas de Zeeu", "thomasdezeeuw@gmail.com"));
        insert("thomcc", a("Thom Chiovoloni", "chiovolonit@gmail.com"));
        insert("timdiekmann", a("Tim Diekmann", "tim.diekmann@3dvision.de"));
        insert("timnn", a("Tim Neumann", "timnn@google.com"));
        insert("tlively", a("Thomas Lively", "tlively@google.com"));
        insert("tmandry", a("Tyler Mandry", "tmandry@gmail.com"));
        insert("tmiasko", a("Tomasz Miąsko", "tomasz.miasko@gmail.com"));
        insert("topecongiro", a("topecongiro", "seuchida@gmail.com"));
        insert("tromey", a("Tom Tromey", "tom@tromey.com"));
        insert("vadimcn", a("Vadim Chugunov", "vadimcn@gmail.com"));
        insert("varkor", a("varkor", "github@varkor.com"));
        insert("veykril", a("Lukas Wirth", "lukastw97@gmail.com"));
        insert("wafflelapkin", a("Waffle Maybe", "waffle.lapkin@gmail.com"));
        insert("weihanglo", a("Weihang Lo", "me@weihanglo.tw"));
        insert("wesleywiser", a("Wesley Wiser", "wwiser@gmail.com"));
        insert("willcrichton", a("Will Crichton", "wcrichto@cs.stanford.edu"));
        insert("withouboats", withoutboats.clone());
        insert("withoutboats", withoutboats.clone());
        insert("wodann", a("Wodann", "wodannson@gmail.com"));
        insert("workingjubilee", a("Jubilee Young", "workingjubilee@gmail.com"));
        insert("wycats", a("Yehuda Katz", "wycats@gmail.com"));
        insert("xampprocky", a("Erin Power", "xampprocky@gmail.com"));
        insert("xanewok", a("Igor Matuszewski", "Xanewok@gmail.com"));
        insert("xfix", a("Konrad Borowski", "konrad@borowski.pw"));
        insert("xfrednet", a("xFrednet", "xFrednet@gmail.com"));
        insert("yaahallo", yaahc.clone());
        insert("yaahc", yaahc.clone());
        insert("yichoi", a("Young-il Choi", "duddlf.choi@samsung.com"));
        insert("y-nak", a("Yoshitomo Nakanishi", "yurayura.rounin.3@gmail.com"));
        insert("yurydelendik", a("Yury Delendik", "ydelendik@mozilla.com"));
        insert("z0w0", a("Zack Corr", "zack@z0w0.me"));
        insert("zackmdavis", a("Zack M. Davis", "code@zackmdavis.net"));
        insert("zoxc", a("John Kåre Alsaker", "john.kare.alsaker@gmail.com"));

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
