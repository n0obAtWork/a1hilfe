# Preiskalkulations-Schema

<!-- source: https://amic.de/hilfe/preiskalkulationsschema.htm -->

Mit dem Modul „Preiskalkulationsschema“ können Kalkulationsschemata für das Preiskalkulationsmodul angelegt und gepflegt werden.

Ein Schema besteht aus

• einer dem Schema eindeutig zugeordneten Schemanummer

• einer Bezeichnung des Schemas

• der Angabe der Kalkulationsgrundlage

• optional die Angabe eines Makros mit Prozeduren zur Kalkulation

• einer Sammlung von Preisberechnungsformeln

• einer Sammlung von Preisbezugsbedingungen.

Der Wert der Kalkulationsgrundlage gibt an, auf welcher Basis Preise kalkuliert werden sollen.

0: Grundpreise aus &lt;KALKLISTENPREIS> zu letztem Zeitraum

1: Zeitraum manuell, Grundpreise=Listenpreise des vorherigen Zeitraumes

Ist dieser Wert 0, so werden bei der Kalkulation die Preise der Spalte „Originalpreis“ aus der Relation „KalkListenPreis“ genommen. Insbesondere können dann auch Artikel kalkuliert werden, die über Preise in dieser Relation verfügen.

Ist der Wert hingegen 1, so entsprechen bei der Kalkulation die Preise der Spalte „Originalpreis“ den aktuellen Listenpreisen aus „ArtiListenPreis“.

Soll für alle Kalkulationsschemata die gleiche Kalkulationsgrundlage gelten, so kann der Wert auch per SPA eingestellt werden. Dann erscheint das Feld nicht mehr auf dieser Maske.

SPA-Bezeichnung: „Preiskalkulation: Kalkulationsgrundlage“ in der Gruppe „Preisfindung“

Werte: 0: Grundpreise aus &lt;KALKLISTENPREIS> zu letztem Zeitraum

 1: Zeitraum manuell, Grundpreise=Listenpreise des vorherigen Zeitraumes

 2: In Kalkulationsschema anzugeben

Die Angabe eines Makro-Namens ist optional. Per SPA kann dieses Feld der Maske unterdrückt werden, wenn generell keine Makros bei der Preiskalkulation verwendet werden sollen.

SPA-Bezeichnung: „Preiskalkulation: Makro-Unterstützung“

Werte: 0: Nein

 1: Ja

Wird hier hingegen ein Makroname angegeben, so muss der entsprechende Makro existieren.

Ein solcher Makro muss dann zu mindestens über die Prozedur

 procedure PreisKalkInit( artikelid,lpgvk,lpgek,pmvk,pmek,ksnr : integer );

(Parameternamen sind dabei natürlich beliebig, die Typen jedoch bindend) verfügen, die jeweils nach Einlesen eines Kalkulationsobjekts ( Listenpreisgruppe ) aufgerufen wird.

Dabei werden in den Parametern folgende Werte übergeben:

Artikelid Die ArtikelId des (eines) Artikels zur aktuellen VK-Listenpreisgruppe

Lpgvk Die aktuelle VK-Listenpreisgruppe

Lpgek Die aktuelle EK-Listenpreisgruppe

Pmvk Die aktuelle VK-Preismatrixnummer

Pmek Die aktuelle EK-Preismatrixnummer

Ksnr Die Nummer des aktuellen Kalkulationsschemas

Gibt es im Makro eine Prozedur

```text
Prozedure
Init();
```

so wird diese noch vor PreisKalkInit aufgerufen. Hier können z.B. global makrointerne Variablen angelegt und initialisiert werden, die dann in PreisKalkInit versorgt wird.

Anwendungsbeispiel:

Sollen in mehreren Formeln eines Preiskalkulationsschemas Werte berücksichtigt werden, die sich etwa in der Relation ArtikelAddOn befinden ( z.B. artikelindividuelle Margen ), so können diese in der Makroprozedur „PreisKalkInit“ gelesen und in durch Init() angelegte globale Makrovariablen festgehalten werden. Gibt es in dem Makro dann etwa eine Funktion

„function MargenWert( lpnr : integer) : real;“

so kann diese in die Formeln eingebaut werden: N1 = N1 + MargenWert(2); N2=N1+MargenWert(N2).

Enthält der Makro eine Prozedure Init(), so sollte er auch eine Prozedur

 Procedure Final();

Enthalten, die am Ende jeder Kalkulation aufgerufen wird. Hier sollte global belegter Speicherplatz wieder freigegeben werden.

Die Bedingungen werden im Kalkulationsmodul vor dem Speichern der Preise zur Prüfung auf Einhaltung herangezogen: Die Preise können erst dann gespeichert werden, wenn die Bedingungen erfüllt sind.

Die Formelsammlung ist in Gruppen unterteilt:

Gruppe 0:

Diese Formeln werden im Kalkulationsmodul zur Berechnung der Neu-Preise beim Aufruf für einen Artikelstamm herangezogen, nicht jedoch bei der manuellen Änderung eines Preises.

Andere Gruppen:

Die Formeln einer Gruppe n (n>0) werden im Kalkulationsmodul ausgeführt, wenn dort der Preis Nn (also der Neu-Preis zum Preis mit Nummer n) manuell überschrieben wird.

Eine Formel besteht aus

einem Formelziel:

Dieses hat immer die Form Nx, wobei x eine Preislistennummer ist, deren Eintrag in der Preislistenrelation über keine Korrektursperre und einer Sortierung (Kalkulation) > 0.00 verfügt.

Dabei darf die Preislistennummer der aktuellen Formelgruppe nicht eingesetzt werden und innerhalb der Gruppe nur einmal als Formelziel erscheinen.

dem Gleichheitszeichen:

Dieses wird vom System automatisch versorgt.

dem Berechnungstext:

Der Berechnungstext ist ein arithmetischer Ausdruck von Operanden, Klammern und Operatoren. Dabei gelten die üblichen Klammerregeln und Operator-Prioritäten.

Zugelassene Operatoren sind {+ (Addition), -(Subtraktion), \* (Multiplikation), / (Division)}. Operanden sind Konstanten, Variablen, Makrofunktionen und wiederum arithmetische Ausdrücke dieser Art.

 Konstanten sind Integer- und Real-Werte. Bei Real-Werten ist die Kommastelle durch einen Punkt zu kennzeichnen.

 Variablen sind von der Form Nx, Ax, oder STGy, wobei x eine Preislistennummer ist, deren Eintrag in der Preislistenrelation über eine Sortierung (Kalkulation) > 0.00 verfügt. Diese können hier auch bei vorhandener Korrektursperre eingesetzt werden.

Ax bezeichnet dabei den ‚alten Preis‘ vor der Kalkulation, Nx den bereits kalkulierten oder manuell geänderten Preis zur Preislistenbezeichnung.

STGy bezeichnet steht für den Steuersatz / 100 (also 0.16 bei 16%), der während der Kalkulation mittels des Steuerschlüssels des kalkulierten Artikels und der Steuergruppe y zum Kalkulationszeitraum bestimmt wird. ‚y‘ muss daher die Nummer einer Steuergruppe sein.

Die Angabe der Steuergruppe kann jedoch unterbleiben; in diesem Fall wird die Steuergruppe laut SPA-Einstellung zur Steuersatzermittlung herangezogen.

 SPA-Bezeichnung: „Preiskalkulation: Default-Steuergruppe“

 Werte: Nummer der Steuergruppe

 Makrofunktionen sind von der Form &lt;Funktionsname>(&lt;Parameterliste>).

&lt;Funktionsname> muss der Funktionsname einer Makro-Funktion im unter Makro-Name bezeichneten Makro sein, die einen Integer- oder Real-Wert zurückgibt.

&lt;Parameterliste> ist eine evtl. leere Aufzählung von Parametern, deren Anzahl der der im Makro definierten Funktion entspricht. Dort sind nur die Parametertypen {integer, real, string} zugelassen. Bei Aufruf der Funktion erfolgt eine typgerechte Konvertierung der Parameterwerte.

 Ein Parameter kann sein:

eine Konstante vom Typ Integer, Real oder String

 Stringkonstanten sind wie “text“ einzuschließen

eine Variable der o.a. Form Nx oder Ax oder STGy

eine der folgenden nur in Parameterlisten erlaubte Zusatzvariablen:

AID ArtikelId des Artikels, dessen Preise gerade kalkuliert werden

LPGV VK-Listenpreisgruppe des Artikes,....

LPGE EK- Listenpreisgruppe des Artikes,....

PMV VK-Preismatrix des Artikes,....

PME EK-Preismatrix des Artikes,....

KSN Nummer des Kalkulkationsschemas

FG Nummer der Formelgruppe, von der aus die Funktion gerade aufgerufen wird

LPN Nummer des Formelziels ( also die Preislistennummer ), die gerade berechnet wird.

ein arithmetischer Ausdruck

Eine Bedingung besteht aus

einer linken Seite:

Dieses hat immer die Form Nx, wobei x eine Preislistennummer ist, deren Eintrag in der Preislistenrelation über eine Sortierung (Kalkulation) > 0.00 verfügt.

Dabei darf die Preislistennummer nur einmal linksseitig erscheinen.

dem Vergleichsoperator:

Zugelassen sind hier die Operatoren {&lt; (kleiner), &lt;= (kleiner oder gleich), > (größer), >= (größer oder gleich)}.

einer rechten Seite:

Dieses ist ein Berechnungstext wie in den Formeln.
