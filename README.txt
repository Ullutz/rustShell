Tema 1 -> Rustybox -> Balteanu Vlad Gabriel

1. pwd
    Am folosit o functie din std::env care imi returneaza un buffer
de tip Path, caruia i-am facut conversie la string si l-am afisat.

2. echo
    Am determinat daca exista flagul -n, apoi am afisat cu cate un
spatiu intre ele, fiecare element al vectorului de argumente citit
de la tastatura.

3. mkdir
    Am folosit functia create_dir pentru a creea cate un director
cu numele citite de la tastatura

4. rmdir
    Am folosit functia remove_dir oentru a sterge un director. In
cazul in care acesta nu este gol, functia intoarce un semnal de eroare.

5. mv
    Functia fs::rename redenumeste un fisier dat stergandu-l apoi
creand unul cu numele nou, implicit la alta cale daca este data.

6. rm
    Aici am folosit mai multe functii. Mai intai daca sterg un fisier
apelez functia rmfile. Daca sterg un director, mai intai verific ca acesta
sa nu fie gol prin functia dir_is_empty. In caz afirmativ, nu pot sterge
decat daca am flagul -r, apeland functia rm_recursive(stiu numele nu este
tocmai sugestiv avand in vedere ca nu este o functie recursiva).

7. ln
    Folosesc doua functii din biblioteca fs:: care imi creeaza un link
hard/ simbolic in cazul in care asa este precizat

8. cat
    Pentru implementarea acestei functii, mai intai am deschis pe rand
fisierele si am citit intregul lor continut, pe care l-am afisat la iesirea
standard.

9. ls
    Pentru fiecare flag am o functie separata.
    -ls simplu : citesc directorul caruia vreau sa-i afisez continutul
si afisez pe rand fiecare obiect din el, daca acesta nu este ascuns
    -ls -a: la fel ca mai sus doar ca afisez si ce este ascuns.
    -ls -r: afisez pe rand fiecare fiecare director, apoi continutul lui
apoi intru pe rand in fiecare din subdirectoare si fac acelasi lucru. =>
un fel de DFS pe grafuri.
    -ls -r -a: acelasi principiu doar ca afisez si ce este ascuns.

10. touch
    La aceasta functie, pentru a modifica timpul de acces intru in fisier
si citesc din el, iar pentru a modifica timpul de modificare (scuzati repetitia)
si scriu ceva in el.
    Daca fisierul nu exista atunci il si creez, daca nu-mi este specificat
explicit ca nu trebuie creat, prin flagul -c.

11. chmod
    Aici am impartit problema in doua cazuri.
    1: daca primesc permisiunile sub forma de numar de 3 cifre. Atunci stiu
ca acesta se afla in baza 8, il transform in baza 10 si apoi il trimit ca
parametru functiei set::permisions.
    2: daca primesc permisiunile sub forma de litere. Aici interpretez
aceste permisiuni si le transform in numar de 3 cifre in baza 8. Apoi
preiau permisiunile deja existente si vad daca: in cazul in care vreau
sa adaug permisiuni, acestea sa nu existe deja, iar daca vreau sa scot 
permsiuni aceast sa existe in prealabil. Astfel modific variabila ce-mi
retine permisiunile pe care vreau sa le adaug/ scot si dupa o adun/scad
din permisiunile deja existente. Apoi transform numarul in baza 10 si il
transmit ca parametru functiei set::permisions.

12. cp
    Daca nu mi se precizeaza o copiere recursiva, nu pot copia directoare.
Daca argumentul dat nu este director atunci il copiez la noua destinatie.
Daca acea destinatie nu exista atunci mai intai o creez si copiez tot ce am nevoie
la noul path.
    Daca mi se precizeaza copierea recursiva atunci, daca am un director intru in el
recursiv si copiez tot ce este de copiat. Daca ajung la un fisier il copiez direct.