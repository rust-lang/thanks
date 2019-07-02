use mailmap::Author;
use std::collections::HashMap;
use std::fmt;

pub struct Reviewers {
    reviewers: HashMap<&'static str, Author>,
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

        let aatch = a("James Miller", "james@aatch.net");
        let alexcrichton = a("Alex Crichton", "alex@alexcrichton.com");
        let gankro = a("Alexis Beingessner", "a.beingessner@gmail.com");
        let huonw = a("Huon Wilson", "wilson.huon@gmail.com");
        let jakub = a("Jakub Kądziołka", "kuba@kadziolka.net");
        let michaelwoerister = a("Michael Woerister", "michaelwoerister@posteo.net");
        let nikomatsakis = a("Niko Matsakis", "niko@alum.mit.edu");
        let nrc = a("Nick Cameron", "ncameron@mozilla.com");
        let pcwalton = a("Patrick Walton", "pcwalton@mimiga.net");
        let steveklabnik = a("Steve Klabnik", "steve@steveklabnik.com");
        let arielb1 = a("Ariel Ben-Yehuda", "ariel.byd@gmail.com");
        let manishearth = a("Manish Goregaokar", "manishsmail@gmail.com");
        let burntsushi = a("Andrew Gallant", "jamslam@gmail.com");
        let guillaumegomez = a("Guillaume Gomez", "guillaume1.gomez@gmail.com");
        let frewsxcv = a("Corey Farwell", "coreyf@rwell.org");
        let brson = a("Brian Anderson", "andersrb@gmail.com");
        let simulacrum = a("Mark Rousskov", "mark.simulacrum@gmail.com");
        let quietmisdreavus = a("QuietMisdreavus", "grey@quietmisdreavus.net");
        let joshtriplett = a("Josh Triplett", "josh@joshtriplett.org");
        let withoutboats = a("Without Boats", "boats@mozilla.com");
        let zoxc = a("John Kåre Alsaker", "john.kare.alsaker@gmail.com");
        let centril = a("Mazdak Farrokhzad", "twingoow@gmail.com");

        map.insert("Aatch", aatch.clone());
        map.insert("Gankro", gankro.clone());
        map.insert("ILyoan", a("ILYONG CHO", "ilyoan@gmail.com"));
        map.insert("manishearth", manishearth.clone());
        map.insert("Manishearth", manishearth.clone());
        map.insert("aatch", aatch.clone());
        map.insert("achrichto", alexcrichton.clone());
        map.insert("acrichto", alexcrichton.clone());
        map.insert("alexcricton", alexcrichton.clone());
        map.insert("alexcrhiton", alexcrichton.clone());
        map.insert("alexcricthon", alexcrichton.clone());
        map.insert("alexchrichton", alexcrichton.clone());
        map.insert("alexcirchton", alexcrichton.clone());
        map.insert("alexcrichto", alexcrichton.clone());
        map.insert("alexcrichton", alexcrichton.clone());
        map.insert("alexcritchton", alexcrichton.clone());
        map.insert("anasazi", a("Eric Reed", "ecreed@cs.washington.edu"));
        map.insert("arielb", arielb1.clone());
        map.insert("arielb1", arielb1.clone());
        map.insert("aturon", a("Aaron Turon", "aturon@mozilla.com"));
        map.insert("bblum", a("Ben Blum", "bblum@andrew.cmu.edu"));
        map.insert("bjz", a("Brendan Zabarauskas", "bjzaba@yahoo.com.au"));
        map.insert("bluss", a("Ulrik Sverdrup", "bluss@users.noreply.github.com"));
        map.insert("bson", brson.clone());
        map.insert("brson", brson.clone());
        map.insert("bstrie", a("Ben Striegel", "ben.striegel@gmail.com"));
        map.insert("burntsushi", burntsushi.clone());
        map.insert("BurntSushi", burntsushi.clone());
        map.insert("catamorphism", a("Tim Chevalier", "chevalier@alum.wellesley.edu"));
        map.insert("chris", a("Chris Morgan", "me@chrismorgan.info"));
        map.insert("cmr", a("Corey Richardson", "corey@octayn.net"));
        map.insert("dotdash", a("Björn Steinbrink", "bsteinbr@gmail.com"));
        map.insert("eddyb", a("Eduard-Mihai Burtescu", "edy.burt@gmail.com"));
        map.insert("erickt", a("Erick Tryzelaar", "erick.tryzelaar@gmail.com"));
        map.insert("flaper87", a("Flavio Percoco", "flaper87@gmail.com"));
        map.insert("gankro", gankro.clone());
        map.insert("graydon", a("Graydon Hoare", "graydon@pobox.com"));
        map.insert("huon", huonw.clone());
        map.insert("huonw", huonw.clone());
        map.insert("jakub", jakub.clone());
        map.insert("jakub-", jakub.clone());
        map.insert("japaric", a("Jorge Aparicio", "jorge@japaric.io"));
        map.insert("jbclements", a("John Clements", "clements@racket-lang.org"));
        map.insert("jdm", a("Josh Matthews", "josh@joshmatthews.net"));
        map.insert("kballard", a("Lily Ballard", "lily@sb.org"));
        map.insert("kmcallister", a("Keegan McAllister", "mcallister.keegan@gmail.com"));
        map.insert("luqmana", a("Luqman Aden", "laden@csclub.uwaterloo.ca"));
        map.insert("metajack", a("Jack Moffitt", "jack@metajack.im"));
        map.insert("mw", michaelwoerister.clone());
        map.insert("michaelwoerister", michaelwoerister.clone());
        map.insert("msullivan", a("Michael J. Sullivan", "sully@msully.net"));
        map.insert("ncr", nrc.clone());
        map.insert("nick29581", nrc.clone());
        map.insert("nikomatsakis", nikomatsakis.clone());
        map.insert("nmatsakis", nikomatsakis.clone());
        map.insert("nrc", nrc.clone());
        map.insert("pcwalton", pcwalton.clone());
        map.insert("pnkfelix", a("Felix S Klock II", "pnkfelix@mozilla.com"));
        map.insert("pwalton", pcwalton.clone());
        map.insert("sanxiyn", a("Seo Sanghyeon", "sanxiyn@gmail.com"));
        map.insert("sfackler", a("Steven Fackler", "sfackler@gmail.com"));
        map.insert("steveklabnik", steveklabnik.clone());
        map.insert("steveklanik", steveklabnik.clone());
        map.insert("steveklbanik", steveklabnik.clone());
        map.insert("thestinger", a("Daniel Micay", "danielmicay@gmail.com"));
        map.insert("yichoi", a("Young-il Choi", "duddlf.choi@samsung.com"));
        map.insert("z0w0", a("Zack Corr", "zack@z0w0.me"));
        map.insert("jroesch", a("Jared Roesch", "roeschinc@gmail.com"));
        map.insert("vadimcn", a("Vadim Chugunov", "vadimcn@gmail.com"));
        map.insert("bkoropoff", a("Brian Koropoff", "bkoropoff@gmail.com"));
        map.insert("Kimundi", a("Marvin Löbel", "loebel.marvin@gmail.com"));
        map.insert("apasel422", a("Andrew Paseltiner", "apaseltiner@gmail.com"));
        map.insert("nagisa", a("Simonas Kazlauskas", "git@kazlauskas.me"));
        map.insert("guillaumegomez", guillaumegomez.clone());
        map.insert("GuillaumeGomez", guillaumegomez.clone());
        map.insert("imperio", guillaumegomez.clone());
        map.insert("jseyfried", a("Jeffrey Seyfried", "jeffrey.seyfried@gmail.com"));
        map.insert("jonathandturner", a("Jonathan Turner", "jturner@mozilla.com"));
        map.insert("gereeter", a("Jonathan S", "gereeter+code@gmail.com"));
        map.insert("frewsxcv", frewsxcv.clone());
        map.insert("frewsxcxv", frewsxcv.clone());
        map.insert("frewsxcvx", frewsxcv.clone());
        map.insert("petrochenkov", a("Vadim Petrochenkov", "vadim.petrochenkov@gmail.com"));
        map.insert("est31", a("est31", "MTest31@outlook.com"));
        map.insert("ollie27", a("Oliver Middleton", "olliemail27@gmail.com"));
        map.insert("rkruppe", a("Robin Kruppe", "robin.kruppe@gmail.com"));
        map.insert("keeperofdakeys", a("Josh Driver", "keeperofdakeys@gmail.com"));
        map.insert("matklad", a("Aleksey Kladov", "aleksey.kladov@gmail.com"));
        map.insert("wycats", a("Yehuda Katz", "wycats@gmail.com"));
        map.insert("eerden", a("Ercan Erden", "ercerden@gmail.com"));
        map.insert("carllerche", a("Carl Lerche", "me@carllerche.com"));
        map.insert("Mark-Simulacrum", simulacrum.clone());
        map.insert("Mark-Simulacru", simulacrum.clone());
        map.insert("mark-simulacrum", simulacrum.clone());
        map.insert("simulacrum", simulacrum.clone());
        map.insert("TimNN", a("Tim Neumann", "timnn@google.com"));
        map.insert("quietmisdreavus", quietmisdreavus.clone());
        map.insert("QuietMisdreavus", quietmisdreavus.clone());
        map.insert("QuietMisdreqvus", quietmisdreavus.clone());
        map.insert("Susurrus", a("Bryant Mairs", "bryant@mai.rs"));
        map.insert("estebank", a("Esteban Küber", "esteban@kuber.com.ar"));
        map.insert("aidanhs", a("Aidan Hobson Sayers", "aidanhs@cantab.net"));
        map.insert("killercup", a("Pascal Hertleif", "killercup@gmail.com"));
        map.insert("cuviper", a("Josh Stone", "jistone@redhat.com"));
        map.insert("lukaramu", a("lukaramu", "lukaramu@users.noreply.github.com"));
        map.insert("carols10cents", a("Carol (Nichols || Goulding)", "carol.nichols@gmail.com"));
        map.insert("kennytm", a("kennytm", "kennytm@gmail.com"));
        map.insert("dtolnay", a("David Tolnay", "dtolnay@gmail.com"));
        map.insert("oli-obk", a("Oliver Scherer", "github35764891676564198441@oli-obk.de"));
        map.insert("Zoxc", zoxc.clone());
        map.insert("zoxc", zoxc.clone());
        map.insert("malbarbo", a("Marco A L Barbosa", "malbarbo@gmail.com"));
        map.insert("withouboats", withoutboats.clone());
        map.insert("withoutboats", withoutboats.clone());
        map.insert("durka", a("Alex Durka", "web@alexburka.com"));
        map.insert("abonander", a("Austin Bonander", "austin.bonander@gmail.com"));
        map.insert("SimonSapin", a("Simon Sapin", "simon.sapin@exyr.org"));
        map.insert("cramertj", a("Taylor Cramer", "cramertj@google.com"));
        map.insert("joshtriplett", joshtriplett.clone());
        map.insert("JoshTriplett", joshtriplett.clone());
        map.insert("KodrAus", a("Ashley Mannix", "ashleymannix@live.com.au"));
        map.insert("Eh2406", a("Eh2406", "YeomanYaacov@gmail.com"));
        map.insert("pietroalbini", a("Pietro Albini", "pietro@pietroalbini.org"));
        map.insert("kornelski", a("Kornel", "kornel@geekhood.net"));
        map.insert("scalexm", a("scalexm", "alexandre@scalexm.fr"));
        map.insert("djc", a("Dirkjan Ochtman", "dirkjan@ochtman.nl"));
        map.insert("gnzlbg", a("gnzlbg", "gonzalobg88@gmail.com"));
        map.insert("centril", centril.clone());
        map.insert("Centril", centril.clone());
        map.insert("matthewjasper", a("Matthew Jasper", "mjjasper1@gmail.com"));
        map.insert("RalfJung", a("Ralf Jung", "post@ralfj.de"));
        map.insert("varkor", a("varkor", "github@varkor.com"));
        map.insert("nnethercote", a("Nicholas Nethercote", "nnethercote@mozilla.com"));
        map.insert("ehuss", a("Eric Huss", "eric@huss.org"));
        map.insert("dwijnand", a("Dale Wijnand", "dale.wijnand@gmail.com"));
        map.insert("collin5", a("Collins Abitekaniza", "abtcolns@gmail.com"));
        map.insert("davidtwco", a("David Wood", "david@davidtw.co"));
        map.insert("jamesmunns", a("James Munns", "james@onevariable.com"));
        map.insert("ljedrz", a("ljedrz", "ljedrz@gmail.com"));
        map.insert("wesleywiser", a("Wesley Wiser", "wwiser@gmail.com"));
        map.insert("MatthewJasper", a("Matthew Jasper", "mjjasper1@gmail.com"));
        map.insert("tmandry", a("Tyler Mandry", "tmandry@gmail.com"));
        map.insert("zackmdavis", a("Zack M. Davis", "code@zackmdavis.net"));
        map.insert("nikic", a("Nikita Popov", "nikita.ppv@gmail.com"));
        map.insert("Amanieu", a("Amanieu d'Antras", "amanieu@gmail.com"));
        map.insert("tromey", a("Tom Tromey", "tom@tromey.com"));
        map.insert("alexreg", a("Alexander Regueiro", "alexreg@me.com"));
        map.insert("Aaronepower", a("Erin Power", "xampprocky@gmail.com"));
        map.insert("euclio", a("Andy Russell", "arussell123@gmail.com"));
        map.insert("phansch", a("Philipp Hansch", "dev@phansch.net"));
        map.insert("flip1995", a("flip1995", "hello@philkrones.com"));
        map.insert("xfix", a("Konrad Borowski", "konrad@borowski.pw"));
        map.insert("matthiaskrgr", a("Matthias Krüger", "matthias.krueger@famsik.de"));
        map.insert("Xanewok", a("Igor Matuszewski", "Xanewok@gmail.com"));
        map.insert("alexheretic", a("Alex Butler", "alexheretic@gmail.com"));
        map.insert("fitzgen", a("Nick Fitzgerald", "fitzgen@gmail.com"));
        map.insert("petrhosek", a("Petr Hosek", "phosek@gmail.com"));
        map.insert("mcarton", a("mcarton", "cartonmartin+git@gmail.com"));
        map.insert("badboy", a("Jan-Erik Rediger", "janerik@fnordig.de"));
        map.insert("scottmcm", a("Scott McMurray", "scottmcm@users.noreply.github.com"));
        map.insert("sunfishcode", a("Dan Gohman", "sunfish@mozilla.com"));
        map.insert("shepmaster", a("Jake Goulding", "jake.goulding@gmail.com"));
        map.insert("Aaron1011", a("Aaron Hill", "aa1ronham@gmail.com"));
        map.insert("bjorn3", a("bjorn3", "bjorn3@users.noreply.github.com"));
        map.insert("mark-i-m", a("Mark Mansi", "markm@cs.wisc.edu"));
        map.insert("spastorino", a("Santiago Pastorino", "spastorino@gmail.com"));
        map.insert("Dylan-DPC", a("dylan_DPC", "dylan.dpc@gmail.com"));
        map.insert("mikerite", a("Michael Wright", "mikerite@lavabit.com"));

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
        ];
        if skip.contains(&reviewer) {
            return Ok(None);
        }
        if let Some(v) = self.reviewers.get(reviewer).cloned() {
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
