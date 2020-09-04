# GESTIONE DI GRAFI DI ASSEMBLAGGIO RUST
La libreria rs-gfa (https://github.com/chfi/rs-gfa) contiene un parser
rust per file GFA. Però la rappresentazione di grafi non è
particolarmente raffinata. Un'altra libreria rilevante è rs-handlegraph
(https://github.com/chfi/rs-handlegraph) che presenta un'interfaccia
rust per un handlegraph: una libreria particolarmente efficiente per la
gestione di grafi in genomica.
L'obiettivo dello stage è estendere rs-gfa affinché utilizzi
direttamente un handlegraph per memorizzare il grafo.
