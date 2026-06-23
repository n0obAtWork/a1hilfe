# Partiebehandlung in MACROS

<!-- source: https://amic.de/hilfe/_partiebehandlunginma.htm -->

Zur Erzeugung von Partieverteilungen im MACRO werden die folgenden Funktionen bereitgestellt:

Hinweis: Bitte bei der Korrektur einer Warenposition **nie** auf dem Original anwenden. Stets *positionausposition* und *replaceposition* benutzen!

```text
function
StartPartieVerteilung( pos_handle: integer ) : integer;
```

Einleitung einer Partieverteilung für eine Warenposition (pos_handle). Vorher teilweise zu Ende geführte Verteilung wird gelöscht.

Ergebnis : immer 1

```text
function AddPartieMenge(
pos_handle: integer; partie_id: integer;
 Partie_artiposit: integer;
 menge: real;
artikel_hinzufuegen : integer) : integer;
```

Hinzufügen einer neuen Partie per (genauere) Identifikation über die PartieId und der PartieArtiPosit. Die Menge muss in der Mengeneinheit des Artikels (Ergebnismenge bei Gebinden!) übergeben werden. Bei artikel_hinzufuegen = 1 wird der Artikel der Partie hinzugefügt, falls noch nicht vorhanden.

Ergebnis 1, wenn erfolgreich

```text
function
AddPartieMengeNummer( pos_handle: integer; partienummer
: integer;
 menge: real;
artikel_hinzufuegen : integer) : integer;
```

Die Angabe der Partie erfolgt über die Nummer. Die Zuordnung kann unter Umständen nicht eindeutig sein. Sonst wie **AddPartieMenge**

```text
function
EndPartieVerteilung(): integer;
```

Erst der erfolgreiche Aufruf dieser Funktion (Ergebnis = 1) setzt die Zuordnung in die Warenposition und überschreibt komplett die bis dahin bestehende Partiezuordnung. Es werden geprüft:

- die Vollständigkeit der Mengenangaben (0 oder Gesamtmenge des Artikels)
- Restbestandsüberprüfung laut FRZ-Einstellung.
