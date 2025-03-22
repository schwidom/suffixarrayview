#![allow(unused)]

use std::{
 cmp::{
  Ordering::{Greater, Less},
  max, min,
 },
 error::Error,
 fmt::Display,
 fs::{File, metadata, read, read_to_string},
 io::Read,
 ops::Deref,
 os::fd,
 process::exit,
 rc::Rc,
};

use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
 // #[arg(
 //  short,
 //  long,
 //  help = "shows equal lines only once, depends on filename and width switch"
 // )]
 // unique: bool,

 // #[arg(short, long, help = "like uniq but prefixes a count number")]
 // count_unique: bool,
 #[arg(short, long, help = "align the output ro the right")]
 right: bool,

 #[arg(
  short,
  long,
  help = "align the output to the left -l and -r is treated as -l"
 )]
 left: bool,

 // iqeg9jggza
 #[arg(short, long, help = "shows the filename indices")]
 index: bool,

 // cczwcifhkq
 #[arg(short, long, help = "shows the filenames")]
 filename: bool,

 // xulmgekuwa
 #[arg(short, long, help = "offset in bytes")]
 offset: bool,

 // tevwc0mwpk
 #[arg(short, long, help = "shows the data before the output datapoint")]
 previous: Option<usize>,

 // gxqxpkgasa
 #[arg(short, long, help = "separator between previous data and data")]
 separator: Option<String>,

 #[arg(
  short,
  long,
  help = "maximum dataset length in chars / bytes, default is unlimited"
 )]
 width: Option<usize>,

 #[arg(short, long, help = "binary mode, default is UTF8 mode")]
 binary: bool,

 // #[arg(
 //  short,
 //  long,
 //  help = "input regex, without this regex every suffix goes to the end of the file"
 // )]
 // regex: Option<String>,

 // #[arg(short, long, help = "converts every '\\n' to ' '")]
 // endoflinetospace: bool,

 // #[arg(short, long, help = "defaults to '\\n'")]
 // datasetend: Option<String>,
 #[arg(
  short,
  long,
  help = "default is every char / byte, defines the start of all suffixes"
 )]
 tokenregex: Option<String>,

 #[arg(short, long)]
 verbose: bool,

 #[arg(long, help = "don't sort")]
 nosort: bool,

 #[arg()]
 files: Vec<String>,
}

#[derive(Clone)]
enum FData<'a> {
 B(&'a FDataBinary<'a>),
 S(&'a FDataString<'a>),
}

impl<'a> FData<'a> {
 fn get_filename(&self) -> &FileName<'a> {
  match self {
   FData::B(fdata_binary) => &fdata_binary.fln,
   FData::S(fdata_string) => &fdata_string.fln,
  }
 }

 fn count(&self) -> usize {
  match self {
   FData::B(fdata_binary) => fdata_binary.data.len(),
   FData::S(fdata_string) => fdata_string.data.chars().count(),
  }
 }

 fn len(&self) -> usize {
  match self {
   FData::B(fdata_binary) => fdata_binary.data.len(),
   FData::S(fdata_string) => fdata_string.data.len(),
  }
 }

 fn create_filepointers(
  &self,
  args: &'a Args,
  tokenregex: &'a Option<Regex>,
 ) -> Result<Box<dyn Iterator<Item = FilePointer<'a>> + 'a>, Box<dyn Error>> {
  match &self {
   FData::B(b) => b.create_filepointers(&args, &tokenregex),
   FData::S(s) => s.create_filepointers(&args, &tokenregex),
  }
 }
}

// TODO : index
#[derive(Clone, Debug)]
struct FileName<'a> {
 idx: usize,
 filename: &'a str,
}

struct FDataBinary<'a> {
 fln: FileName<'a>,
 data: Vec<u8>,
}

struct FDataString<'a> {
 fln: FileName<'a>,
 data: String,
 // utf8data: Vec<char>,
 char_to_byte_offset: Vec<usize>,
 byte_to_char_offset: Vec<usize>,
}

impl<'a> FDataBinary<'a> {
 fn new(fln: FileName<'a>) -> Result<Self, Box<dyn Error>> {
  let data = match read(&fln.filename) {
   Ok(value) => value,
   Err(e) => {
    eprintln!("{} : {}", &fln.filename, e);
    exit(1);
   }
  };
  Ok(Self { fln, data })
 }

 fn create_filepointers(
  &'a self,
  args: &'a Args,
  tokenregex: &'a Option<Regex>,
 ) -> Result<Box<dyn Iterator<Item = FilePointer<'a>> + 'a>, Box<dyn Error>> {
  // TODO : fehler, wenn tokenregex != None ist

  if args.verbose {
   eprintln!("creating suffixes {}", self.fln.filename);
  }

  let l = self.data.len();
  Ok(Box::new((0..l).map(|offset| FilePointer {
   fdata: FData::B(self),
   byte_offset: offset,
  })))
 }
}

impl<'a> FDataString<'a> {
 fn new(fln: FileName<'a>) -> Result<Self, Box<dyn Error>> {
  let data = match read_to_string(&fln.filename) {
   Ok(value) => value,
   Err(e) => {
    eprintln!("{} : {}", &fln.filename, e);
    exit(1);
   }
  };
  Self::from_bogus(fln, data)
 }

 fn from_bogus(fln: FileName<'a>, data: String) -> Result<Self, Box<dyn Error>> {
  // let data = read_to_string(&filename)?;
  // let utf8data = data.chars().collect::<Vec<_>>();
  let mut char_to_byte_offset: Vec<_> = data.char_indices().map(|x| x.0).collect();
  char_to_byte_offset.push(data.len()); // endidx

  let mut byte_to_char_offset: Vec<usize> = vec![];
  {
   let r_end = *char_to_byte_offset.last().unwrap_or(&0usize);
   // let byte_to_char_offset = char_to_byte_offset
   for (char_offset, byte_offset) in char_to_byte_offset.iter().enumerate() {
    while byte_to_char_offset.len() <= *byte_offset {
     byte_to_char_offset.push(char_offset);
    }
   }
  }

  Ok(Self {
   fln,
   data,
   // utf8data,
   char_to_byte_offset,
   byte_to_char_offset,
  })
 }

 fn create_filepointers(
  &'a self,
  args: &'a Args,
  tokenregex: &'a Option<Regex>,
 ) -> Result<Box<dyn Iterator<Item = FilePointer<'a>> + 'a>, Box<dyn Error>> {
  if let Some(tr) = tokenregex {
   if args.verbose {
    eprintln!("creating suffixes with token regex {}", self.fln.filename);
   }
   let l = self.data.chars().count();
   let it = tr.find_iter(&self.data);
   Ok(Box::new(it.map(|e| {
    let res = self.filepointer_byteoffset(e.range().start);
    res
   })))
  } else {
   if args.verbose {
    eprintln!("creating suffixes {}", self.fln.filename);
   }

   let l = self.data.chars().count();
   Ok(Box::new((0..l).map(|offset| self.filepointer_charoffset(offset))))
  }
 }

 fn byte_to_char_offset(&self, w: usize) -> usize {
  if w < self.byte_to_char_offset.len() {
   self.byte_to_char_offset[w]
  } else {
   let l = self.byte_to_char_offset.last();
   match l {
    Some(l) => *l,
    None => 0,
   }
  }
 }

 fn char_to_byte_offset(&self, w: usize) -> usize {
  if w < self.char_to_byte_offset.len() {
   self.char_to_byte_offset[w]
  } else {
   match self.char_to_byte_offset.last() {
    Some(l) => *l,
    None => 0,
   }
  }
 }
}

impl<'a> FDataString<'a> {
 fn filepointer_byteoffset(&self, byte_offset: usize) -> FilePointer {
  FilePointer {
   fdata: FData::S(self),
   byte_offset,
  }
 }
 fn filepointer_charoffset(&self, char_offset: usize) -> FilePointer {
  let byte_offset = self.char_to_byte_offset[char_offset];
  FilePointer {
   fdata: FData::S(&self),
   byte_offset,
  }
 }
}

#[derive(Clone)]
struct FilePointer<'a> {
 fdata: FData<'a>,
 byte_offset: usize,
}
impl<'a> FilePointer<'a> {
 fn char_offset(&self) -> usize {
  match self.fdata {
   FData::B(_) => self.byte_offset,
   FData::S(s) => s.byte_to_char_offset(self.byte_offset),
  }
 }
}

pub fn main() -> Result<(), Box<dyn Error>> {
 let args = Args::parse();
 // println!("{:?}", args.files);

 if 0 == args.files.len() {
  eprintln!("no inputfiles given");
  exit(1);
 }

 for filename in &args.files {
  let md = metadata(filename);
  match md {
   Err(e) => {
    eprintln!("{} : {}", e, filename);
    exit(1);
   }
   Ok(md) => {
    if md.is_dir() {
     eprintln!("Is a directory : {}", filename);
     exit(1);
    }
   }
  }
 }

 let mut filedata = vec![];

 for (idx, filename) in args.files.iter().enumerate() {
  if args.verbose {
   eprintln!("reading {}", filename);
  }

  let fln = FileName { idx, filename };
  if args.binary {
   filedata.push(FData::B(Box::leak(Box::new(FDataBinary::new(fln)?))));
  } else {
   filedata.push(FData::S(Box::leak(Box::new(FDataString::new(fln)?))));
  }
 }

 let tokenregex: Option<Regex> = match args.tokenregex.clone() {
  Some(tokenregex) => Some(Regex::new(&tokenregex)?),
  None => None,
 };

 // let mut filepointers = filedata
 //  .iter()
 //  .flat_map(|fdata| -> Result<Vec<FilePointer>,Box<dyn Error>> { fdata.create_filepointers(&args)?.deref()})
 //  .collect::<Result<Vec<FilePointer>,Box<dyn Error>>>();

 let mut filepointers = filedata
  .iter()
  .flat_map(|fdata| {
   fdata
    .create_filepointers(&args, &tokenregex)
    .unwrap()
    .into_iter()
  })
  .collect::<Vec<FilePointer>>();

 if !args.nosort {
  if args.verbose {
   eprintln!("sorting");
  }

  filepointers.sort_by(|a, b| match (&a.fdata, &b.fdata) {
   (FData::B(a1), FData::B(b1)) => a1.data[a.byte_offset..].cmp(&b1.data[b.byte_offset..]),
   (FData::B(a1), FData::S(b1)) => Less,
   (FData::S(a1), FData::B(b1)) => Greater,
   (FData::S(a1), FData::S(b1)) => a1.data[a.byte_offset..].cmp(&b1.data[b.byte_offset..]),
  });
 }

 let alignments = {
  if args.right || args.left {
   let mut res = vec![];

   for fd in &filedata {
    let fln = fd.get_filename();

    let idxlen = format!("{}", fln.idx).chars().count();
    let fnlen = format!("{}", fln.filename).chars().count();
    res.push((idxlen, fnlen));
   }

   let idxlenmax = res.iter().map(|x| x.0).max().unwrap();
   let fnlenmax = res.iter().map(|x| x.1).max().unwrap();

   Some(
    res
     .iter()
     .map(|x| (" ".repeat(idxlenmax - x.0), " ".repeat(fnlenmax - x.1)))
     .collect::<Vec<_>>(),
   )
  } else {
   None
  }
 };

 let offset_alignment_base = {
  if args.left || args.right {
   filedata.iter().map(|x| format!("{}", x.len()).len()).max()
  } else {
   None
  }
 };

 // output
 for filepointer in filepointers {
  let data2print_previous = match args.previous {
   Some(prev) => Some(match filepointer.fdata {
    // TODO : hex output
    FData::B(b) => {
     let r_start = if filepointer.byte_offset > prev { filepointer.byte_offset - prev } else { 0 };
     format!("{:?}", &b.data[r_start..filepointer.byte_offset])
    }
    FData::S(s) => {
     let co = filepointer.char_offset();
     let r_start = s.char_to_byte_offset(if co > prev { co - prev } else { 0 });

     format!(
      "{}",
      s.data[r_start..filepointer.byte_offset]
       .to_string()
       .replace("\n", " ")
     )
    }
   }),
   None => None,
  };

  let data2print = match filepointer.fdata {
   // TODO : hex output
   FData::B(b) => {
    let r_end = match args.width {
     Some(w) => max(filepointer.byte_offset, min(b.data.len(), filepointer.byte_offset + w)),
     None => b.data.len(),
    };
    format!("{:?}", &b.data[filepointer.byte_offset..r_end])
   }
   FData::S(s) => {
    let r_end = match args.width {
     Some(w) => s.char_to_byte_offset(filepointer.char_offset() + w),
     None => s.data.len(),
    };

    format!(
     "{}",
     s.data[filepointer.byte_offset..r_end]
      .to_string()
      .replace("\n", " ")
    )
   }
  };

  // let filename = match filepointer.fdata {
  //  FData::B(b) => b.filename,
  //  FData::S(s) => s.filename,
  // };

  let fln = filepointer.fdata.get_filename();

  if args.index {
   if (args.left || args.right) {
    if let Some(al) = &alignments {
     print!("{}", al[fln.idx].0)
    }
   }
   // iqeg9jggza

   print!("{} ", fln.idx);
  }

  if args.filename {
   if let Some(al) = &alignments {
    if args.right && !args.left {
     print!("{}", al[fln.idx].1)
    }
   }

   // cczwcifhkq
   print!("{} ", fln.filename);

   if let Some(al) = &alignments {
    if args.left {
     print!("{}", al[fln.idx].1)
    }
   }
  }

  if args.offset {
   // xulmgekuwa
   let toprint = format!("{}", filepointer.byte_offset);
   if let Some(oab) = offset_alignment_base {
    print!("{}", " ".repeat(oab - toprint.chars().count()))
   }
   print!("{} ", toprint);
  }

  // gxqxpkgasa
  if let Some(d2p) = data2print_previous {
   let d2p = format!("{}", d2p);

   if !args.binary && (args.left || args.right) {
    if let Some(prev) = args.previous {
     let rep = if prev > d2p.len() { prev - d2p.len() } else { 0 };
     print!("{}", " ".repeat(rep));
    }
   }

   print!("{} ", d2p);
  }

  if let Some(sep) = &args.separator {
   // gxqxpkgasa
   print!("{} ", sep);
  }

  println!("{}", data2print);
 }

 Ok(())
}
#[cfg(test)]
mod tests {
 use crate::libmain::FileName;

 use super::FDataString;

 #[test]
 fn test_001() {
  let s = "⡌⠁⠧⠑ ⠼⠁⠒  ⡍⠜⠇⠑⠹⠰⠎ ⡣⠕⠌";
  let ci: Vec<_> = s.char_indices().map(|x| x.0).collect();
  assert_eq!(
   &vec![
    0usize, 3, 6, 9, 12, 13, 16, 19, 22, 23, 24, 27, 30, 33, 36, 39, 42, 45, 46, 49, 52
   ],
   &ci
  );
 }

 #[test]
 fn test_002() {
  let s = "⡌⠁⠧⠑ ⠼⠁⠒  ⡍⠜⠇⠑⠹⠰⠎ ⡣⠕⠌"; // chars : 21
  let fds = FDataString::from_bogus(
   FileName {
    idx: 0,
    filename: "testfile",
   },
   s.to_owned(),
  )
  .unwrap();
  assert_eq!(
   &vec![
    0usize, 3, 6, 9, 12, 13, 16, 19, 22, 23, 24, 27, 30, 33, 36, 39, 42, 45, 46, 49, 52, 55
   ],
   &fds.char_to_byte_offset
  );
  assert_eq!(
   &vec![
    0usize, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 10, 11, 11, 11,
    12, 12, 12, 13, 13, 13, 14, 14, 14, 15, 15, 15, 16, 16, 16, 17, 17, 17, 18, 19, 19, 19, 20, 20,
    20, 21, 21, 21
   ],
   &fds.byte_to_char_offset
  );

  let _: &[i32] = &vec![][0..0];
  assert_eq!(fds.data.len(), 55);
  assert_eq!(fds.data.chars().count(), 21); // ok

  assert_eq!(fds.char_to_byte_offset.len(), 22); // ok, mit endidx (55)
  assert_eq!(fds.byte_to_char_offset.len(), 56); // ?

  // let _ = fds.data[0..20]; // panic ( byte index 20 is not a char boundary )
  // let _ = fds.data[0..56];  // panic
  let _ = fds.data[0..55];
  // let _ = fds.data.as_bytes()[0..56]; // panic
  let _ = fds.data.as_bytes()[0..55];

  for &i in &fds.char_to_byte_offset {
   let _ = fds.data[0..i];
  }
  for &i in &fds.byte_to_char_offset {
   let k = &fds.char_to_byte_offset[i];
   let _ = fds.data[0..*k];
  }
 }
}
