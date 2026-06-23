# IF Statement

<!-- source: https://amic.de/hilfe/ifstatement.htm -->

<p class="just-emphasize">Syntax</p>

If ( | DOMAIN(table-name,column-name[,[typ=nnn,len=nnn][,scale=nn]]) |

 | TAB(table-name) |

 | ROWS |

 | INDEX(index-name) |

 | DBERR |

 | VAL(field-name) )

{

 ...

}  
ELSE

{

 ...

}

<p class="just-emphasize">Purpose</p>

Einfache Strukturierungsmöglichkeit;

<p class="just-emphasize">Anwendung</p>

Kommandodatei

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[GOTO](./goto_und_label_statement.md), [EXIT](./exit_statement.md)

<p class="just-emphasize">Beschreibung</p>

Logische Abfrage um auf bestimmte Ereignisse zu reagieren. Hierfür gibt es bestimmte Schlüsselwörter:

DOMAIN(Tabelle,Feld)

Wenn das Feld in der Datenbank existiert ==> TRUE

DOMAIN(Tabelle,Feld,typ=xxx,len=nnn)

Wenn das Feld in der Datenbank existiert und der Typ und die Länge  
stimmen==> TRUE. Typ kann sein:  
integer, smallint, char, long varchar, varchar, binary, time, timestamp, date, double, float, long binary, var binary, tiny int, unsigned int, bit

DOMAINE(Tabelle,Feld,scale=nn)

Wenn das Feld existiert und die Anzahl Nachkommastellen stimmt ==> TRUE

TABLE(table-name)

Wenn die Relation table-name existiert ==> TRUE

ROWS==nnn 

Wobei nnn eine beliebige Zahl ist. Prüft ob das vorangegangene Select - Statement die angegebene Zahl zurückliefert. Mögliche Vergleichsoperatoren sind : ==, >=, &lt;=, &lt;> oder !=, >, &lt;. Dies sollte verwendet werden um zu prüfen, ob überhaupt Daten vorhanden sind ( If ROWS==0 ). Es nützt nichts, den DBERR abzufragen, da dieser auch beim einem Select, dass Daten zurückliefert, DBERR=100 liefert. Das liegt daran, dass alle Sätze gelesen werden und nach dem letzten Satz natürlich der Fehler 100 auftritt.

DBERR==nnn 

Wobei nnn eine beliebige Zahl ist. DBERR ist der von SYBASE zurückgelieferte Fehler ( z.B. –196 für INDEX NOT UNIQUE). Mögliche Vergleichsoperatoren sind : ==, >=, &lt;=, &lt;> oder !=, >, &lt;.

**INDEX(index-name)**  
Wenn der Index mit diesem Namen existiert ==> TRUE.

**VAL(maskenfeld)==Wert  
**Wenn ein Maskenfeld oder LDB-Feld einen bestimmten Wert annimmt==> TRUE. Mögliche Vergleichsoperatoren sind : ==, >=, &lt;=, &lt;> oder !=, >, &lt;.

<p class="just-emphasize">Beispiel</p>

Select \* from fibuvorgstamm where fibuv_nummer is NULL;

IF (ROWS==0) // Keine Daten gefunden

{

 Pause Keine Daten mit NULL;

 EXIT;

}

else

{

 Update Fibuvorgstamm set FiBuV_Nummer=’War leer’ where FiBuV_Nummer Is NULL;

}
