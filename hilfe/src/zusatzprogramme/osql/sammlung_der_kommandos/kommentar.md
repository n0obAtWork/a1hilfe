# Kommentar

<!-- source: https://amic.de/hilfe/kommentar.htm -->

#### Syntax

// Einzeiliger kommentar

/\*

 Mehzeiliger Kommentar

\*/

#### Purpose

Nicht benötigte Statements auskommentieren / Befehlsfolgen Dokumentieren.

#### Anwendung

Kommandodatei

#### Berechtigung

Alle Anwender

#### Beschreibung

Um Kommandodateien auch später noch zu verstehen ist es Sinnvoll entsprechende Kommentare einzufügen. Einzeilige Kommentare können mit mehrzeiligen geschachtelt vorkommen. Ineinander verschachtelte mehrzeilige Kommentare sind nicht möglich.

#### Beispiel

/\* Entwickler OH 

 Datum 20.06.2000-06-20

// Noch ein Kommentar!

\*/ 

// RICHTIG

/\* Entwickler OH 

 Datum 20.06.2000-06-20

/\* Noch ein Kommentar!\*/

\*/ 

//FLASCH, da der erste Kommentar hinter dem ersten „\*/„ aufhört!
