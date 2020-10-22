# PRINCIPALI DIFFERENZE TRA LE DUE IMPLEMENTAZIONI
In generale, a livello di API le differenze maggiori che esistono tra ```libhandlegraph``` e ```rs-handlegraph``` sono 
che quest'ultima e' ancora in fase di sviluppo, mentre la prima e' gia in fase "gold" e necessita solamente di alcune
rifiniture.\
\
```rs-handlegraph``` ha implementato tutte funzioni principali di ```libhandlegraph``` tralasciando per ora le funzioni
"secondarie".\
Una delle quali e' in *WIP (Work In Progress)* ed e' ```MutableHandleGraph```. \
\
Probabilmente insieme a  ```MutableHandleGraph``` si potrebbe implementare ```DeletableHandleGraph``` visto che sono 
due funzioni che lavorano bene insieme.
Anche ```MutablePathHandleGraph``` e' una funzione da considerare di aggiungere insieme alle 2 sopracitate.\
\
Infine ```SerializableHandleGraph``` puo' essere interessante da implementare vista la forte presenza di file ```GFA1```
molto grandi che quindi vengono codificati in ```HandleGraph``` altrettanto grandi.\
Questa grande dimensione dei grafi puo' risultare poco maneggievole e flessibile quando bisogna salvare i dati del 
grafo all'interno di un file. Soprattutto se bisogna salvare molteplici grafi.