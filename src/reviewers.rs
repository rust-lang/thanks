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
        let amanieu = a("Amanieu d'Antras", "amanieu@gmail.com");
        let arielb1 = a("Ariel Ben-Yehuda", "ariel.byd@gmail.com");
        let brson = a("Brian Anderson", "andersrb@gmail.com");
        let burntsushi = a("Andrew Gallant", "jamslam@gmail.com");
        let centril = a("Mazdak Farrokhzad", "twingoow@gmail.com");
        let cramertj = a("Taylor Cramer", "cramertj@google.com");
        let ecstaticmorse = a("Dylan MacKenzie", "ecstaticmorse@gmail.com");
        let frewsxcv = a("Corey Farwell", "coreyf@rwell.org");
        let gankro = a("Alexis Beingessner", "a.beingessner@gmail.com");
        let guillaumegomez = a("Guillaume Gomez", "guillaume1.gomez@gmail.com");
        let hanna_kruppe = a("Hanna Kruppe", "hanna.kruppe@gmail.com");
        let huonw = a("Huon Wilson", "wilson.huon@gmail.com");
        let jakub = a("Jakub Kądziołka", "kuba@kadziolka.net");
        let joshtriplett = a("Josh Triplett", "josh@joshtriplett.org");
        let jyn514 = a("Joshua Nelson", "jyn514@gmail.com");
        let kodraus = a("Ashley Mannix", "ashleymannix@live.com.au");
        let llogiq = a("Andre Bogus", "bogusandre@gmail.com");
        let manishearth = a("Manish Goregaokar", "manishsmail@gmail.com");
        let michaelwoerister = a("Michael Woerister", "michaelwoerister@posteo.net");
        let nikomatsakis = a("Niko Matsakis", "niko@alum.mit.edu");
        let nrc = a("Nick Cameron", "ncameron@mozilla.com");
        let oli_obk = a("Oliver Scherer", "github35764891676564198441@oli-obk.de");
        let pcwalton = a("Patrick Walton", "pcwalton@mimiga.net");
        let petrochenkov = a("Vadim Petrochenkov", "vadim.petrochenkov@gmail.com");
        let quietmisdreavus = a("QuietMisdreavus", "grey@quietmisdreavus.net");
        let ralfjung = a("Ralf Jung", "post@ralfj.de");
        let simulacrum = a("Mark Rousskov", "mark.simulacrum@gmail.com");
        let steveklabnik = a("Steve Klabnik", "steve@steveklabnik.com");
        let withoutboats = a("Without Boats", "boats@mozilla.com");
        let yaahc = a("Jane Lusby", "jlusby42@gmail.com");
        let zoxc = a("John Kåre Alsaker", "john.kare.alsaker@gmail.com");

        map.insert("Aaron1011", a("Aaron Hill", "aa1ronham@gmail.com"));
        map.insert("Aaronepower", a("Erin Power", "xampprocky@gmail.com"));
        map.insert("aatch", aatch.clone());
        map.insert("Aatch", aatch.clone());
        map.insert("abonander", a("Austin Bonander", "austin.bonander@gmail.com"));
        map.insert("achrichto", alexcrichton.clone());
        map.insert("acrichto", alexcrichton.clone());
        map.insert("aDotInTheVoid", a("Nixon Enraght-Moony", "nixon.emoony@gmail.com"));
        map.insert("aidanhs", a("Aidan Hobson Sayers", "aidanhs@cantab.net"));
        map.insert("aleksator", a("Alex Tokarev", "aleksator@gmail.com"));
        map.insert("alexchrichton", alexcrichton.clone());
        map.insert("alexcirchton", alexcrichton.clone());
        map.insert("alexcrhiton", alexcrichton.clone());
        map.insert("alexcrichto", alexcrichton.clone());
        map.insert("alexcrichton", alexcrichton.clone());
        map.insert("alexcricthon", alexcrichton.clone());
        map.insert("alexcricton", alexcrichton.clone());
        map.insert("alexcritchton", alexcrichton.clone());
        map.insert("alexheretic", a("Alex Butler", "alexheretic@gmail.com"));
        map.insert("alexreg", a("Alexander Regueiro", "alexreg@me.com"));
        map.insert("Amaneiu", amanieu.clone());
        map.insert("Amanieu", amanieu.clone());
        map.insert("anasazi", a("Eric Reed", "ecreed@cs.washington.edu"));
        map.insert("apasel422", a("Andrew Paseltiner", "apaseltiner@gmail.com"));
        map.insert("apiraino", a("apiraino", "apiraino@users.noreply.github.com"));
        map.insert("arielb1", arielb1.clone());
        map.insert("arielb", arielb1.clone());
        map.insert("arthurprs", a("arthurprs", "arthurprs@gmail.com"));
        map.insert("aturon", a("Aaron Turon", "aturon@mozilla.com"));
        map.insert("badboy", a("Jan-Erik Rediger", "janerik@fnordig.de"));
        map.insert("bblum", a("Ben Blum", "bblum@andrew.cmu.edu"));
        map.insert("bjorn3", a("bjorn3", "bjorn3@users.noreply.github.com"));
        map.insert("bjz", a("Brendan Zabarauskas", "bjzaba@yahoo.com.au"));
        map.insert("bkoropoff", a("Brian Koropoff", "bkoropoff@gmail.com"));
        map.insert("bluss", a("Ulrik Sverdrup", "bluss@users.noreply.github.com"));
        map.insert("brson", brson.clone());
        map.insert("bson", brson.clone());
        map.insert("bstrie", a("Ben Striegel", "ben.striegel@gmail.com"));
        map.insert("bugadani", a("Dániel Buga", "bugadani@gmail.com"));
        map.insert("burntsushi", burntsushi.clone());
        map.insert("BurntSushi", burntsushi.clone());
        map.insert("BoxyUwU", a("Boxy", "supbscripter@gmail.com"));
        map.insert("c410-f3r", a("Caio", "c410.f3r@gmail.com"));
        map.insert("calebcartwright", a("Caleb Cartwright", "caleb.cartwright@outlook.com"));
        map.insert("camelid", a("Camelid", "camelidcamel@gmail.com"));
        map.insert("camsteffen", a("Cameron Steffen", "cam.steffen94@gmail.com"));
        map.insert("carllerche", a("Carl Lerche", "me@carllerche.com"));
        map.insert("carols10cents", a("Carol (Nichols || Goulding)", "carol.nichols@gmail.com"));
        map.insert("catamorphism", a("Tim Chevalier", "chevalier@alum.wellesley.edu"));
        map.insert("CDirkx", a("Christiaan Dirkx", "christiaan@dirkx.email"));
        map.insert("centril", centril.clone());
        map.insert("Centril", centril.clone());
        map.insert("chris", a("Chris Morgan", "me@chrismorgan.info"));
        map.insert("cjgillot", a("Camille GILLOT", "gillot.camille@gmail.com"));
        map.insert("Cldfire", a("Jarek Samic", "cldfire3@gmail.com"));
        map.insert("cmr", a("Corey Richardson", "corey@octayn.net"));
        map.insert("collin5", a("Collins Abitekaniza", "abtcolns@gmail.com"));
        map.insert("CraftSpider", a("Rune Tynan", "runetynan@gmail.com"));
        map.insert("cramert", cramertj.clone());
        map.insert("cramertj", cramertj.clone());
        map.insert("csmoe", a("csmoe", "csmoe@msn.com"));
        map.insert("cuviper", a("Josh Stone", "jistone@redhat.com"));
        map.insert("danielhenrymantilla", a("Daniel Henry-Mantilla", "daniel.henry.mantilla@gmail.com"));
        map.insert("davidtwco", a("David Wood", "david@davidtw.co"));
        map.insert("dingelish", a("Yu Ding", "dingelish@gmail.com"));
        map.insert("djc", a("Dirkjan Ochtman", "dirkjan@ochtman.nl"));
        map.insert("dns2utf8", a("Stefan Schindler", "dns2utf8@estada.ch"));
        map.insert("dotdash", a("Björn Steinbrink", "bsteinbr@gmail.com"));
        map.insert("dtolnay", a("David Tolnay", "dtolnay@gmail.com"));
        map.insert("durka", a("Alex Durka", "web@alexburka.com"));
        map.insert("dwijnand", a("Dale Wijnand", "dale.wijnand@gmail.com"));
        map.insert("Dylan-DPC", a("dylan_DPC", "dylan.dpc@gmail.com"));
        map.insert("ebroto", a("Eduardo Broto", "ebroto@tutanota.com"));
        map.insert("ecstatic-morse", a("Dylan MacKenzie", "ecstaticmorse@gmail.com"));
        map.insert("ecstatic-morse", ecstaticmorse.clone());
        map.insert("ecstaticmorse", ecstaticmorse.clone());
        map.insert("eddyb", a("Eduard-Mihai Burtescu", "edy.burt@gmail.com"));
        map.insert("eerden", a("Ercan Erden", "ercerden@gmail.com"));
        map.insert("Eh2406", a("Eh2406", "YeomanYaacov@gmail.com"));
        map.insert("ehuss", a("Eric Huss", "eric@huss.org"));
        map.insert("elichai", a("Elichai Turkel", "elichai.turkel@gmail.com"));
        map.insert("erickt", a("Erick Tryzelaar", "erick.tryzelaar@gmail.com"));
        map.insert("est31", a("est31", "MTest31@outlook.com"));
        map.insert("estebank", a("Esteban Küber", "esteban@kuber.com.ar"));
        map.insert("euclio", a("Andy Russell", "arussell123@gmail.com"));
        map.insert("fee1-dead", a("Deadbeef", "ent3rm4n@gmail.com"));
        map.insert("fitzgen", a("Nick Fitzgerald", "fitzgen@gmail.com"));
        map.insert("flaper87", a("Flavio Percoco", "flaper87@gmail.com"));
        map.insert("flip1995", a("flip1995", "hello@philkrones.com"));
        map.insert("frewsxcv", frewsxcv.clone());
        map.insert("frewsxcvx", frewsxcv.clone());
        map.insert("frewsxcxv", frewsxcv.clone());
        map.insert("gankro", gankro.clone());
        map.insert("Gankro", gankro.clone());
        map.insert("gereeter", a("Jonathan S", "gereeter+code@gmail.com"));
        map.insert("giraffate", a("Takayuki Nakata", "f.seasons017@gmail.com"));
        map.insert("gnzlbg", a("gnzlbg", "gonzalobg88@gmail.com"));
        map.insert("graydon", a("Graydon Hoare", "graydon@pobox.com"));
        map.insert("guillaumegomez", guillaumegomez.clone());
        map.insert("GuillaumeGomez", guillaumegomez.clone());
        map.insert("GuilliameGomez", guillaumegomez.clone());
        map.insert("GuilliaumeGomez", guillaumegomez.clone());
        map.insert("hanna-kruppe", hanna_kruppe.clone());
        map.insert("hellow554", a("Marcel Hellwig", "github@cookiesoft.de"));
        map.insert("huon", huonw.clone());
        map.insert("huonw", huonw.clone());
        map.insert("ILyoan", a("ILYONG CHO", "ilyoan@gmail.com"));
        map.insert("imperio", guillaumegomez.clone());
        map.insert("jackh726", a("jackh726", "jack.huey@umassmed.edu"));
        map.insert("jakub", jakub.clone());
        map.insert("jakub-", jakub.clone());
        map.insert("jamesmunns", a("James Munns", "james@onevariable.com"));
        map.insert("japaric", a("Jorge Aparicio", "jorge@japaric.io"));
        map.insert("Jarcho", a("Jason Newcomb", "jsnewcomb@pm.me"));
        map.insert("jbclements", a("John Clements", "clements@racket-lang.org"));
        map.insert("jdm", a("Josh Matthews", "josh@joshmatthews.net"));
        map.insert("jethrogb", a("Jethro Beekman", "jethro@fortanix.com"));
        map.insert("jhpratt", a("Jacob Pratt", "jacob@jhpratt.dev"));
        map.insert("JohnTitor", a("Yuki Okushi", "huyuumi.dev@gmail.com"));
        map.insert("jonas-schievink", a("Jonas Schievink", "jonasschievink@gmail.com"));
        map.insert("jonathandturner", a("Jonathan Turner", "jturner@mozilla.com"));
        map.insert("joshtriplett", joshtriplett.clone());
        map.insert("JoshTriplett", joshtriplett.clone());
        map.insert("jroesch", a("Jared Roesch", "roeschinc@gmail.com"));
        map.insert("jseyfried", a("Jeffrey Seyfried", "jeffrey.seyfried@gmail.com"));
        map.insert("jsgf", a("Jeremy Fitzhardinge", "jsgf@fb.com"));
        map.insert("jsha", a("Jacob Hoffman-Andrews", "github@hoffman-andrews.com"));
        map.insert("jyn514", jyn514.clone());
        map.insert("jyn541", jyn514.clone());
        map.insert("kballard", a("Lily Ballard", "lily@sb.org"));
        map.insert("keeperofdakeys", a("Josh Driver", "keeperofdakeys@gmail.com"));
        map.insert("kennytm", a("kennytm", "kennytm@gmail.com"));
        map.insert("killercup", a("Pascal Hertleif", "killercup@gmail.com"));
        map.insert("Kimundi", a("Marvin Löbel", "loebel.marvin@gmail.com"));
        map.insert("kinnison", a("Daniel Silverstone", "dsilvers@digital-scurf.org"));
        map.insert("kmcallister", a("Keegan McAllister", "mcallister.keegan@gmail.com"));
        map.insert("kodraus", kodraus.clone());
        map.insert("kodrAus", kodraus.clone());
        map.insert("KodrAus", kodraus.clone());
        map.insert("kornelski", a("Kornel", "kornel@geekhood.net"));
        map.insert("lcnr", a("Bastian Kauschke", "bastian_kauschke@hotmail.de"));
        map.insert("LeSeulArtichaut", a("LeSeulArtichaut", "leseulartichaut@gmail.com"));
        map.insert("LingMan", a("LingMan", "LingMan@users.noreply.github.com"));
        map.insert("ljedrz", a("ljedrz", "ljedrz@gmail.com"));
        map.insert("llogiq", llogiq.clone());
        map.insert("llogic", llogiq.clone());
        map.insert("lnicola", a("Laurențiu Nicola", "lnicola@dend.ro"));
        map.insert("Lokathor", a("Lokathor", "zefria@gmail.com"));
        map.insert("lqd", a("lqd", "remy.rakic+github@gmail.com"));
        map.insert("lukaramu", a("lukaramu", "lukaramu@users.noreply.github.com"));
        map.insert("LukasKalbertodt", a("Lukas Kalbertodt", "lukas.kalbertodt@gmail.com"));
        map.insert("luqmana", a("Luqman Aden", "laden@csclub.uwaterloo.ca"));
        map.insert("lzutao", a("Lzu Tao", "taolzu@gmail.com"));
        map.insert("malbarbo", a("Marco A L Barbosa", "malbarbo@gmail.com"));
        map.insert("manishearth", manishearth.clone());
        map.insert("Manishearth", manishearth.clone());
        map.insert("Manisheart", manishearth.clone());
        map.insert("mark-i-m", a("Mark Mansi", "markm@cs.wisc.edu"));
        map.insert("mark-simulacrum", simulacrum.clone());
        map.insert("mark-Simulacrum", simulacrum.clone());
        map.insert("Mark-Simulacrum", simulacrum.clone());
        map.insert("Mark-Simulacru", simulacrum.clone());
        map.insert("mark-simulcrum", simulacrum.clone());
        map.insert("MarkSimulacrum", simulacrum.clone());
        map.insert("marmeladema", a("marmeladema", "xademax@gmail.com"));
        map.insert("mati865", a("Mateusz Mikuła", "mati865@gmail.com"));
        map.insert("matklad", a("Aleksey Kladov", "aleksey.kladov@gmail.com"));
        map.insert("matthewjasper", a("Matthew Jasper", "mjjasper1@gmail.com"));
        map.insert("MatthewJasper", a("Matthew Jasper", "mjjasper1@gmail.com"));
        map.insert("matthiaskrgr", a("Matthias Krüger", "matthias.krueger@famsik.de"));
        map.insert("max-heller", a("max-heller", "max.a.heller@gmail.com"));
        map.insert("mcarton", a("mcarton", "cartonmartin+git@gmail.com"));
        map.insert("metajack", a("Jack Moffitt", "jack@metajack.im"));
        map.insert("michaelwoerister", michaelwoerister.clone());
        map.insert("mikerite", a("Michael Wright", "mikerite@lavabit.com"));
        map.insert("mikeyhew", a("Michael Hewson", "michael@michaelhewson.ca"));
        map.insert("mjbshaw", a("Michael Bradshaw", "mjbshaw@google.com"));
        map.insert("m-ou-se", a("Mara Bos", "m-ou.se@m-ou.se"));
        map.insert("msullivan", a("Michael J. Sullivan", "sully@msully.net"));
        map.insert("mw", michaelwoerister.clone());
        map.insert("Nadrieril", a("Nadrieril", "nadrieril+git@gmail.com"));
        map.insert("nagisa", a("Simonas Kazlauskas", "git@kazlauskas.me"));
        map.insert("nbdd0121", a("Gary Guo", "gary@garyguo.net"));
        map.insert("ncr", nrc.clone());
        map.insert("nellshamrell", a("Nell Shamrell-Harrington", "nellshamrell@gmail.com"));
        map.insert("Nemo157", a("Wim Looman", "git@nemo157.com"));
        map.insert("nick29581", nrc.clone());
        map.insert("nikic", a("Nikita Popov", "nikita.ppv@gmail.com"));
        map.insert("nikomatsakis", nikomatsakis.clone());
        map.insert("nmatsakis", nikomatsakis.clone());
        map.insert("nnethercote", a("Nicholas Nethercote", "nnethercote@mozilla.com"));
        map.insert("notriddle", a("Michael Howell", "michael@notriddle.com"));
        map.insert("nrc", nrc.clone());
        map.insert("obi-obk", oli_obk.clone());
        map.insert("oli-obk", oli_obk.clone());
        map.insert("oli", oli_obk.clone());
        map.insert("ollie27", a("Oliver Middleton", "olliemail27@gmail.com"));
        map.insert("pcwalton", pcwalton.clone());
        map.insert("pczarn", a("Piotr Czarnecki", "pioczarn@gmail.com"));
        map.insert("peschkaj", a("Jeremiah Peschka", "jeremiah.peschka@gmail.com"));
        map.insert("petrhosek", a("Petr Hosek", "phosek@gmail.com"));
        map.insert("petrochencov", petrochenkov.clone());
        map.insert("petrochenkov", petrochenkov.clone());
        map.insert("phansch", a("Philipp Hansch", "dev@phansch.net"));
        map.insert("pickfire", a("Ivan Tham", "pickfire@riseup.net"));
        map.insert("pietroalbini", a("Pietro Albini", "pietro@pietroalbini.org"));
        map.insert("pnkfelix", a("Felix S Klock II", "pnkfelix@mozilla.com"));
        map.insert("poliorcetics", a("Alexis Bourget", "alexis.bourget@gmail.com"));
        map.insert("pwalton", pcwalton.clone());
        map.insert("quietmisdreavus", quietmisdreavus.clone());
        map.insert("QuietMisdreavus", quietmisdreavus.clone());
        map.insert("QuietMisdreqvus", quietmisdreavus.clone());
        map.insert("ralfjung", ralfjung.clone());
        map.insert("RalfJung", ralfjung.clone());
        map.insert("raoulstrackx", a("Raoul Strackx", "raoul.strackx@fortanix.com"));
        map.insert("rbtcollins", a("Robert Collins", "robertc@robertcollins.net"));
        map.insert("retep998", a("Peter Atashian", "retep998@gmail.com"));
        map.insert("richkadel", a("Rich Kadel", "richkadel@google.com"));
        map.insert("rkruppe", hanna_kruppe.clone());
        map.insert("rylev", a("Ryan Levick", "me@ryanlevick.com"));
        map.insert("sanxiyn", a("Seo Sanghyeon", "sanxiyn@gmail.com"));
        map.insert("scalexm", a("scalexm", "alexandre@scalexm.fr"));
        map.insert("scottmcm", a("Scott McMurray", "scottmcm@users.noreply.github.com"));
        map.insert("seanmonstar", a("Sean McArthur", "sean@seanmonstar.com"));
        map.insert("sfackler", a("Steven Fackler", "sfackler@gmail.com"));
        map.insert("shepmaster", a("Jake Goulding", "jake.goulding@gmail.com"));
        map.insert("SimonSapin", a("Simon Sapin", "simon.sapin@exyr.org"));
        map.insert("simulacrum", simulacrum.clone());
        map.insert("skade", a("Florian Gilcher", "florian.gilcher@asquera.de"));
        map.insert("spastorino", a("Santiago Pastorino", "spastorino@gmail.com"));
        map.insert("steveklabnik", steveklabnik.clone());
        map.insert("steveklanik", steveklabnik.clone());
        map.insert("steveklbanik", steveklabnik.clone());
        map.insert("stjepang", a("Stjepan Glavina", "stjepang@gmail.com"));
        map.insert("Stupremee", a("Justus K", "justus.k@protonmail.com"));
        map.insert("sunfishcode", a("Dan Gohman", "sunfish@mozilla.com"));
        map.insert("Susurrus", a("Bryant Mairs", "bryant@mai.rs"));
        map.insert("Swatinem", a("Arpad Borsos", "swatinem@swatinem.de"));
        map.insert("the8472", a("The8472", "git@infinite-source.de"));
        map.insert("thestinger", a("Daniel Micay", "danielmicay@gmail.com"));
        map.insert("Thomasdezeeuw", a("Thomas de Zeeu", "thomasdezeeuw@gmail.com"));
        map.insert("TimDiekmann", a("Tim Diekmann", "tim.diekmann@3dvision.de"));
        map.insert("TimNN", a("Tim Neumann", "timnn@google.com"));
        map.insert("tlively", a("Thomas Lively", "tlively@google.com"));
        map.insert("tmandry", a("Tyler Mandry", "tmandry@gmail.com"));
        map.insert("tmiasko", a("Tomasz Miąsko", "tomasz.miasko@gmail.com"));
        map.insert("topecongiro", a("topecongiro", "seuchida@gmail.com"));
        map.insert("tromey", a("Tom Tromey", "tom@tromey.com"));
        map.insert("vadimcn", a("Vadim Chugunov", "vadimcn@gmail.com"));
        map.insert("varkor", a("varkor", "github@varkor.com"));
        map.insert("wesleywiser", a("Wesley Wiser", "wwiser@gmail.com"));
        map.insert("withouboats", withoutboats.clone());
        map.insert("withoutboats", withoutboats.clone());
        map.insert("Wodann", a("Wodann", "wodannson@gmail.com"));
        map.insert("workingjubilee", a("Jubilee Young", "workingjubilee@gmail.com"));
        map.insert("wycats", a("Yehuda Katz", "wycats@gmail.com"));
        map.insert("XAMPPRocky", a("Erin Power", "xampprocky@gmail.com"));
        map.insert("Xanewok", a("Igor Matuszewski", "Xanewok@gmail.com"));
        map.insert("xfix", a("Konrad Borowski", "konrad@borowski.pw"));
        map.insert("xFrednet", a("xFrednet", "xFrednet@gmail.com"));
        map.insert("yaahallo", yaahc.clone());
        map.insert("yaahc", yaahc.clone());
        map.insert("Yaahc", yaahc.clone());
        map.insert("yichoi", a("Young-il Choi", "duddlf.choi@samsung.com"));
        map.insert("Y-Nak", a("Yoshitomo Nakanishi", "yurayura.rounin.3@gmail.com"));
        map.insert("yurydelendik", a("Yury Delendik", "ydelendik@mozilla.com"));
        map.insert("z0w0", a("Zack Corr", "zack@z0w0.me"));
        map.insert("zackmdavis", a("Zack M. Davis", "code@zackmdavis.net"));
        map.insert("zoxc", zoxc.clone());
        map.insert("Zoxc", zoxc.clone());

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
