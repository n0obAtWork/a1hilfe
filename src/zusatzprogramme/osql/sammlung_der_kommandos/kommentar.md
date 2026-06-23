# Kommentar

<!-- source: https://amic.de/hilfe/kommentar.htm -->

<p class="just-emphasize">Syntax</p>

// Einzeiliger kommentar

/\*

 Mehzeiliger Kommentar

\*/

<p class="just-emphasize">Purpose</p>

Nicht benötigte Statements auskommentieren / Befehlsfolgen Dokumentieren.

<p class="just-emphasize">Anwendung</p>

Kommandodatei

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Beschreibung</p>

Um Kommandodateien auch später noch zu verstehen ist es Sinnvoll entsprechende Kommentare einzufügen. Einzeilige Kommentare können mit mehrzeiligen geschachtelt vorkommen. Ineinander verschachtelte mehrzeilige Kommentare sind nicht möglich.

<p class="just-emphasize">Beispiel</p>

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
