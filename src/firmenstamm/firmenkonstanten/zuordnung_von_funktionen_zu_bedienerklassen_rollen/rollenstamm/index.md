# Rollenstamm

<!-- source: https://amic.de/hilfe/_rollenstamm.htm -->

Hauptmenü > Administration > Firmenkonstanten > Zugriffsrechte Rolle > Rollenstamm

oder Direktsprung **[ROLLE]**

Es ergibt sich für jeden Kontext „Listen“ von Bedienerklassen, die die Funktion ausführen dürfen. Eine Rolle ist ein Synonym für eine solche Liste. Es ist eine bis zu 255 Zeichen umfassende freiwählbare Zeichenkette, die innerhalb der Rollen eindeutig sein muss.

Es ist möglich für jeden Kontext eine eigene Rolle anzulegen, da aber für den Großteil der Funktionen sich in der Praxis gleiche Rollen ergeben, wird man sich für diese auf eine Rolle einigen wollen.

<details>
<summary>Felder des Rollenstamm</summary>

| Felder | Beschreibung |
| --- | --- |
| Rolle | Eindeutiger Bezeichner für eine Rolle. Bis zu 255 Zeichen.  
Die Bezeichner sind nach der Erstinitialisierung technisch anmutend durchnummeriert: R000R, R001R, R002R, … usw. Diese Bezeichnung hat den Vorteil in anderen Auswahllisten leichter auffindbar zu sein.  
Es gibt eine nicht durch ihren Namen, der ist ebenso frei wählbar, aber durch ihre Funktionalität ausgezeichnete Rolle: Die sogenannte Controller-Rolle. Diese Controller-Rolle ist die Rolle, die neuen Kontexten -also Funktionen, die zu Kontexten hinzugefügt werden oder auch per Neuanlage oder Update ins System kommen- zugeordnet werden. Das System unterbindet das Löschen dieser Controller-Rolle bzw. stellt auch ggf. sicher das eine solche Controller-Rolle existiert, falls von Nöten. Eine Controller-Rolle muss es geben, um neue Funktionalitäten zunächst einmal nur eine dafür vorgesehenen Bedienerschaft zugänglich zu machen. |
| Anzahl Bedienerklassen | Informatorische Anzahl der zugeordneten Bedienerklassen zu dieser Rolle.  
Es kann Rollen ohne Bedienerklassen geben. Ordnet man einem Kontext eine solche Rolle zu darf kein Anwender die betreffende Funktion in dem Kontext ausführen. |
| Anzahl Kontexte | Informatorische Anzahl der zugeordneten Kontexte zu dieser Rolle.  
Je nach Einsatzgebiet der Rolle kann diese Anzahl 18.000+ oder auch nur sehr wenige bis gar keine Kontexte umfassen. |
| Bedienerklassen dürfen | Informatorische Schutzstring-Auflistung der „positiven“ Bedienerklassen.  
Die Darstellung ist gerafft, d.d. aufeinanderfolgende Bedienerklassen sind zusammengefasst.  
Aber Achtung: 1-3 bedeutet die Bedienerklassen 1, 2 und 3 – wenn es denn überhaupt eine Bedienerklasse 2 gibt. Sonst eben nur die Bedienerklassen 1 und 3. Aber das sollte keine wirkliche Einschränkung darstellen. |
| Bedienerklassen dürfen nicht | Informatorische Schutzstring der Bedienerklassen die nicht der Rolle zugeordnet sind.  
Rundet die Übersicht in Kombination mit „Bedienerklassen dürfen“ ab: Je nach Fragestellung kann man sich in der passenden Umgebung schnell orientieren. |

</details>

<details>
<summary>Suchfunktionen des Rollenstamm</summary>

| Suchkriterien |
| --- |
| Rolle | Suchbegriff |

</details>

<details>
<summary>Funktionen des Rollenstamm</summary>

| Funktionen |
| --- |
| Neu (**F8**) | Anlage einer neuen Rolle.  
Für Details siehe [Rollenpfleger](./rollenstamm_pfleger.md). |
| Ändern (**F5**) | Ändern einer Rolle  
Als Besonderheit beinhaltet diese Funktion auch das „Umbenennen einer Rolle, sowie über „Speichern unter“ das Erzeugen einer neuen Rolle mit Vorgabe der Bedienerklassenzuordnung der Vorlage.  
Für Details siehe Rollenpfleger. |
| Ansehen (**F6**) | Ansehen der Bedienerklassenzuordnung.  
Für Details siehe [Rollenpfleger](./rollenstamm_pfleger.md). |
| Löschen (**F7**) | Löscht eine Rolle.  
Allerdings nicht die Controller-Rolle und ebenso keine Rolle die zugeordnete Rollenkontexte hat!  
Für Details siehe [Rollenpfleger](./rollenstamm_pfleger.md). |
| Rollen tauschen (**F9**) | Hiermit lassen sich von zwei Rollen die Bedienerklassenzuordnung und/oder die Rollenkontexte austauschen.  
Für Details siehe „[Rollen tauschen](./rollen_tauschen.md)“. |
| Rollen vereinigen (**Shift+F9**) | Vereinigt ausgewählte Rolle in einer Ziel-Rolle. Die Ziel-Rolle kann dabei entweder eine neue oder eine der beteiligten Rollen sein. Nach der Operation besitzt die Ziel-Rolle die Vereinigung aller Bedienerklassen der beteiligten Rollen und alle betroffenen Rollenkontexte der involvierten Rollen sind der Zielrolle zugeordnet.  
Für Details siehe „[Rollen vereinigen](./rollen_vereinigen.md)“. |
| Rollenkontexte (**F10**) | Ruft die Anwendung „Diese Funktionen“ auf, die eine Spezialisierung einer Variante der Rollenkontexte ist und nur die Mitglieder dieser Rolle anzeigt. |

</details>

<p class="siehe-auch">Siehe auch:</p>

- [Rollenstamm: Pfleger](./rollenstamm_pfleger.md)
- [Rollen tauschen](./rollen_tauschen.md)
- [Rollen vereinigen](./rollen_vereinigen.md)
