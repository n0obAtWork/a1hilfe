# Rollenpflegerstamm

<!-- source: https://amic.de/hilfe/rollenpflegerstamm.htm -->

Hauptmenü > Administration > Firmenkonstanten > Zugriffsrechte Rollenpflegerstamm

oder

Hauptmenü > Stammdatenpflege > Stammdatenpfleger > Zugriffsrechte Rollenpflegerstamm

oder Direktsprung [ROPST]

Die Zugriffsrechte der jeweiligen Pfleger ergeben sich aus den Zugriffsrechten jeweiliger dedizierter Kontexte, den sogenannten „bestimmenden Kontexten“ oder auch der „Pfleger-Rollenbindung“.

Wird zur Laufzeit ein Pfleger angefordert, so entscheidet der zugeordnete Kontext, ob eine Anforderung erlaubt ist.

Besteht keine Autorisierung durch den bestimmenden Rollenkontext wird das Laufzeitsystem den Anwender informieren und zwecks administrativer Unterstützung eine Warnung ins Fehlerprotokoll eingestellt.

<details>
<summary>Felder des Rollenpflegerstamm</summary>

| Felder |
| --- |
| Pflegerstamm | [Pflegerstamm](../../../zusatzprogramme/pflegerstamm.md) |
| Besitzer | Dieses Feld wird z.Z. nicht angezeigt.  
Momentan sind keine anderen Besitzer von Pflegerstämmen als AMIC vorgesehen. |
| Methode | Pflegerstamm-Methode  
Mögliche Ausprägungen sind  
• Neu  
• Ändern  
• Ansehen  
• Löschen |
| Optionbox | Die Optionbox des bestimmenden Kontextes |
| Funktion | Die Funktion des bestimmenden Kontextes |
| Rolle | Die Rolle des bestimmenden Kontextes |

</details>

<details>
<summary>Suchmöglichkeiten Rollenpflegerstamm</summary>

| Suchkriterien | |
| --- | --- |
| Suchen | Sucht in den Feldern  
• Pflegerstamm  
• Optionbox  
• Funktion  
• Rolle |
| Methode | Sucht im Feld „Methode“ |

</details>

<details>
<summary>Funktionen:</summary>

| Funktionen | |
| --- | --- |
| Funktion Informationen (**F9**) | Aufruf eines [Informationsdialoges zur Funktion](./rollenkontext/rollenkontext_pfleger.md). |
| Funktion ansehen/bearbeiten (**F11**) | Aufruf des Anwendfunktions-Pflegers. |
| Kontext … (**F10**) | Aufruf des Optionbox-Pflegers (steht ausschließlich der Entwicklung zur Verfügung) |
| | | |

</details>
