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

/// importo librerie per gestione file, path, directory
/// e gestione input/output (3 ottobre, 2020)
use std::fs::File;
use std::io::prelude::*;
extern crate glob;
use glob::glob;
use std::path::Path;
use std::io;

/// definisco una funzione di stampa con gestione errori
/// (3 ottobre, 2020)
fn print(result: Result<String, io::Error>) {
    // in base al risultato stampo un messaggio di errore oppure stampo il risultato desiderato
    match result {
        Ok(file) => println!("File content:\n{}\n", file),
        Err(why) => println!("Error: {}\n", why),
    }
}

/// definisco una funzione che legge il contenuto di un file passato in input
/// (3 ottobre, 2020)
fn read_file(filename: &str) -> Result<String, io::Error> {
    // tento di aprire un file da un path passato in input
    let mut file = File::open(filename)?;

    // leggo un file e lo salvo il contenuto come string
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // ritorno il contenuto del file
    Ok(content)
}

/// definisco una funzione che lista tutti i file all'interno di una data directory
/// con controllo e gestione errori (anche se non funziona bene il display di questi ultimi)
/// (3 ottobre, 2020)
fn read_directory_files(dirname: &str) -> Vec<String> {
    // creo un vettore di String in cui salvare i risultati della scansione
    let mut files:Vec<String> = vec![];

    // scansiono tutta la directory
    for e in glob(dirname).unwrap() {
        match e {
            Ok(path) => {
                // creo una variabile in cui inserisco i vari nomi dei file (con tanto di relative path)
                let filename = path.display().to_string();

                match filename.as_str() {
                    // essendo che i file seguenti sono molto grandi, evito di inserirli e stamparli
                    // solo per comodita' di visualizzazione
                    // penso che non sia il modo migliore ne piu' bello per gestire questa casistica, ma
                    // ora come ora funziona e non so come fare altrimenti
                    "test\\gfas\\very_big_file.gfa" =>
                        println!("The file {} is too big! so we do not display it!", "test\\gfas\\very_big_file.gfa"),
                    "test\\gfas\\very_big_file2.gfa" =>
                        println!("The file {} is too big! so we do not display it!\n", "test\\gfas\\very_big_file2.gfa"),
                    // inserisco il nome dei file che trovo nella directory all'interno del vettore
                    // files, convertendo i risultati della scansione da Result<> a String
                    _ => files.push(filename),
                }
            },
            // questo branch di match non stampa il messaggio di errore, probabilmente
            // e' legato al valore di ritorno che essendo contenuto in files, se files e'
            // empty allora non stampa nulla, ma essendoci una println! dovrebbe almeno stampare
            // quella. idk
            Err(why) => println!("Error: {}", why),
        }
    }
    // ritorno il vettore contenente il risultato come espressione di ritorno
    // ricorda che le espressioni di ritorno NON vogliono il ";" finale
    files
}

fn main() {
    // test funzione read_file su un file inesistente per test errore
    let filepath = "test/gfas/i_dont_exist.gfa";
    println!("Call function: read_file\n\nFilename: {}", filepath);
    print(read_file(filepath));

    // test funzione read_directory_files e successivamente
    // test funzione read_file con loop
    let dirpath = "test/gfas/*";
    println!("Call function: read_directory_files\n");

    let files: Vec<String> = read_directory_files(dirpath);
    // doppio for per estrarre le informazioni da files
    for file in files {
        println!("Filename: {}", file);
        print(read_file(&*file));
    }
}
