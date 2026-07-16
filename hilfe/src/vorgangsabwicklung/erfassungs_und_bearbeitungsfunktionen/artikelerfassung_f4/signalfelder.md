# Signalfelder

<!-- source: https://amic.de/hilfe/_warenpositionerfassen_signalfelder.htm -->

Es gibt Zustände einer Warenposition, die verschiedene buchungstechnische Folgen haben oder Informationen, die dem Erfasser deutlich sichtbar gemacht werden sollen. Diese werden mit farbigen Signalfeldern angezeigt. Bisher z.T. auf der Maske angezeigte Hinweistexte wurden durch diese Signalfelder ersetzt.

### Allgemeine Signalfelder

| Name | Beschreibung |
| --- | --- |
| Wertartikel | Dieses Signal wird angezeigt, wenn es sich bei der Artikelposition um einen Wertartikel handelt.<br>Dieser Status konnte bisher nur durch Anwahl einer Funktion analog zur Artikelerfassung angegeben werden. Nun ist auch während der Artikelerfassung das Ein-/Ausschalten dieser Information möglich. |
| Stückl. Hauptartikel | (nur geplant) Dieser Artikel ist ein Hauptartikel einer Stückliste |
| Stückl. Folgeartikel | (nur geplant) Dieser Artikel ist ein Folgeartikel einer Stückliste |
| Hauptartikel | (nur geplant) Dieser Artikel ist Hauptartikel einer Artikelfolge |
| Folgeartikel | (nur geplant) Dieser Artikel ist ein Folgeartikel einer Artikelfolge |
| (kein) Kontraktartikel | (nur geplant) Dieses Signal gibt an, dass dieser Artikel einem Kontrakt zugeordnet ist |
| (kein) Partieartikel | (nur geplant) Dieses Signal zeigt an, dass diesem Artikel eine Partie zuzuordnen ist |
| (keine) Lagerabholung | Hier wird angezeigt, dass dieser Artikel zu denen gehören soll, die auf der Lieferung als Abholung im Lager gekennzeichnet werden sollen. |
| Ausbuchen | Hier wird angezeigt, ob bei der „[Schnellen Teildisposition](./schnelle_teildisposition.md)“ die Position ausgebucht werden soll |

### Signalfelder aus Voreinkauf, Vorverkauf, Einlagerung und Kommission

Alle diese Felder lassen sich zum Zeitpunkt der (Erst-)Erfassung mit Hilfe von Menüfunktionen ein- und ausschalten (toggeln).

| Anzeige | Seite | Beschreibung |
| --- | --- | --- |
| Vorverkauf | Verkauf | Vorverkauf.<br>Dieser Status konnte bisher nur durch Anwahl einer Funktion analog zur Artikelerfassung angegeben werden. Nun ist auch während der Artikelerfassung das Ein-/Ausschalten dieser Information möglich. |
| Abholung VVK | Verkauf | Abholung eines Vorverkaufs. |
| Vorverk. Rück | Verkauf | (nur geplant) Rücknahme eines Vorverkaufs (vor Lieferung) |
| Voreinkauf | Einkauf | Voreinkauf.<br>Dieser Status konnte bisher nur durch Anwahl einer Funktion analog zur Artikelerfassung angegeben werden. Nun ist auch während der Artikelerfassung das Ein-/Ausschalten dieser Information möglich. |
| Anlieferung | Einkauf | Anlieferung voreingekaufter Ware |
| Voreink. Rück | Einkauf | (nur geplant) Rückgabe voreingekaufter Ware (vor Anlieferung) |
| Kommission | Verkauf | Ausgabe der Ware als Kommissionsware. |
| VK d. Kommission | Verkauf | Verkauf von Kommissions-Ware. |
| Rückn d. Kommission | Verkauf | (nur geplant) Rücknahme von als Kommissionsware ausgegebener Ware |
| Einlagerung | Einkauf | Einlagerung fremder Ware ins eigene Lager |
| Herausgabe | Einkauf | Abholung einer Einlagerung durch den Eigentümer |
| Vereinnahmen | Einkauf | Vereinnahmung eingelagerter Ware |

### Weitere Signalfelder

| Anzeige | Beschreibung |
| --- | --- |
| aus Umwandlung | Zeigt an, dass die Position aus einer Belegumwandlung (zum Beispiel Rechnung aus Lieferschein) entstanden ist.<br> |
| Aus Teildisposition | Zeigt an, dass die Position per Teildisposition aus einem Vorbeleg entstanden ist. |
| Nullpreis OK | Dieses Signalfeld zeigt an, dass der Anwender dieses Signalfeld bewusst über die Funktion aus der Optionbox „[Nullpreis Okay an/aus](./merkmale.md#Nullpreis_anaus)“ gesetzt hat, um zu kennzeichnen, dass der angegebene Preis 0 wirklich 0 sein soll. Das sichtbare Signalfeld ist für den Anwender ein Hinweis darauf, dass die 0 im Feld Preis bewusst eingetragen wurde und nicht etwa vergessen wurde den Preis anzugeben.<br>Durch erneutes Anwählen der Funktion „[Nullpreis Okay an/aus](./merkmale.md#Nullpreis_anaus)“ wird dieses Signalfeld wieder entfernt. |
| Nicht Endpreis | Dieses Signalfeld zeigt an; dass der Anwender dieses Signalfeld bewusst über die Funktion aus der Optionbox „[Vorläufiger Preis an/aus](./merkmale.md#VorlaeufigerPreis_anaus)“ gesetzt hat, um zu kennzeichnen, dass der angegebene Preis ein vorläufiger Preis sein soll und noch nicht der Endpreis ist. Das sichtbare Signalfeld ist für den Anwender ein Hinweis darauf, dass im Feld Preis noch etwas zu tun ist.<br>Durch erneutes Anwählen der Funktion „[Vorläufiger Preis an/aus](./merkmale.md#VorlaeufigerPreis_anaus)“ wird dieses Signalfeld wieder entfernt. |
