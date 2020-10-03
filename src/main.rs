/// definizione delle varie funzioni per l'interfacciamento del
/// progetto rs-gfa con il progetto rs-handlegraph per far si che
/// rs-gfa che si occupa di conversione di un grafo gfa si interfacci
/// con rs-handlegraph che si occupa della rappresentazione di un grafo
/// gfa in memoria

/// importo librerie esterne github
use handlegraph;
use gfa;

/// importo librerie per leggere e gestire input da linea di comando
extern crate clap;
use clap::{App, Arg};

/// include per gestione file, path e directory (3 ottobre, 2020)
use std::fs::File;
use std::io::prelude::*;
extern crate glob;
use glob::glob;
use std::path::Path;


/// definisco una funzione che legge il contenuto di un file passato in input
/// (3 ottobre, 2020)
fn read_file(filename: &str) -> () {
    // tento di aprire un file da un path passato in input
    let mut file = File::open(filename).expect("Can't open the file!");

    // leggo un file e lo salvo il contenuto come string
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Can't read the file!");

    // metto a display il contenuto del file
    println!("read_file function result:\n{}\n", content);
}

/// definisco una funzione che lista tutti i file all'interno di una data directory
/// (3 ottobre, 2020)
fn read_directory_files(dirname: &str) -> Vec<String> {
    // creo un vettore di String in cui salvare i risultati della scansione
    let mut files:Vec<String> = vec![];

    // scansiono tutta la directory
    for e in glob(dirname).expect("Failed to read global pattern") {
        // creo una variabile in cui inserisco i vari nomi dei file (con tanto di relative path)
        let mut filename = e.unwrap().display().to_string();
        
        match filename.as_str() {
            // essendo che i file seguenti sono molto grandi, evito di inserirli e stamparli
            // solo per comodita' di visualizzazione
            // penso che non sia il modo migliore ne piu' bello per gestire questa casistica, ma
            // ora come ora funziona e non so come fare altrimenti
            "test\\gfas\\very_big_file.gfa" =>
                println!("The file {} is too big! so we do not display it!\n", "test\\gfas\\very_big_file.gfa"),
            "test\\gfas\\very_big_file2.gfa"=>
                println!("The file {} is too big! so we do not display it!\n", "test\\gfas\\very_big_file2.gfa"),
            // inserisco il nome dei file che trovo nella directory all'interno del vettore
            // files, convertendo i risultati della scansione da Result<> a String
            _ => files.push(filename),
        }
    }

    // ritorno il vettore contenente il risultato come espressione di ritorno
    // ricorda che le espressioni di ritorno NON vogliono il ";" finale
    files
}

fn main() {
    println!("Hello, world!\n");

    // test funzione read_file con singolo file
    let filepath = "test/gfas/big.gfa";
    println!("Content of the file: {}", filepath);
    read_file(filepath);

    // test funzione read_directory_files e successivamente
    // test funzione read_file con loop
    let dirpath = "test/gfas/*";
    println!("Call function read_directory_files");

    let mut files: Vec<String> = read_directory_files(dirpath);
    for file in files.iter() {
        println!("Content of the file: {}", file);
        read_file(file);
    }
}
