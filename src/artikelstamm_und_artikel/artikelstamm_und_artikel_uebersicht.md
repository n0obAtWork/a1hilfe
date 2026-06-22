# Artikelstamm und Artikel — Übersicht

<!-- source: https://amic.de/hilfe/_artikelstamm_und_artikeluebersicht.htm -->

In den Artikelstammdaten werden alle Informationen über einen Artikel zusammengefasst, auf die für eine weitgehend automatisierte Verarbeitung zugegriffen werden muss. Dies sind z.B. Mengeneinheiten, Preise, Gebindegröße etc. Da zur Vereinfachung der Erfassung bei der Anlage eines Artikels auf vorerfasste Informationen zugegriffen wird, müssen diese natürlich vorher angelegt worden sein. So wird sicherlich häufig die Mengeneinheit "Stück" benötigt. Diese muss also zuvor in der Tabelle "Mengeneinheiten" erfasst werden.

Vor der Erfassung der Artikelkonten sollten also verschiedene Konstanten vorher eingegeben sein, da auf sie bei der Stammdatenerfassung zugegriffen wird.  
Aus der Sicht des Artikelstamms sind dies:

• Mengeneinheitsgruppen und Mengeneinheiten

• Warengruppen

• Steuersätze

• Erlöskennziffern

Darüber hinaus können weitere Konstanten in Abhängigkeit von der Anwendung hinzukommen. So sind die Gefahrgutkennzeichen zu erfassen, wenn die Gefahrgutabwicklung aktiviert werden soll.

Bei den Artikeln wird unterschieden zwischen Artikelstamm **[ARS]** und Artikel **[AR]**.

Der Artikel stellt die bebuchbare Einheit dar, z.B. das Konto „Flasche Weißwein“ auf einem Lager. Lagerübergreifend weisen bebuchbare Artikel jedoch zwingend Gemeinsamkeiten auf: Für eine gemeinsame Bestandsführung müssen sie die gleiche Mengeneinheit besitzen und der gleichen Warengruppe angehören; auch das Ge­wicht ist natürlich gleich. Diese Daten werden im Artikelstamm zusammengefasst. Für die Erfassung ergeben sich dadurch folgende Abläufe:

Wenn nur ein Lager vorhanden ist, erfolgt die Stammdatenerfassung über den Anwahlpunkt Artikel **[AR]**, wo dann alle Informationen in einem Ablauf erfasst werden

Sind Artikel auf mehreren Lagern vorhanden, ist es sinnvoll über die Artikelstammerfassung zu arbeiten. Es werden dann zuerst die übergreifenden Informationen eingegeben und dann aus der Erfassungsmaske heraus die individuellen Daten pro Artikel

Über die Steuerungsparameter werden für die konkrete Anwendung Zentraleinstellungen vorgenommen. Dies können z.B. Aktivierungen von Stammdatenfunktionen sein (Folgeartikel aktiv), Vorbelegungen mit Werten, Definition von Obergrenzen (Länge Artikelnummer) sein. Auf die Bedeutung dieser Parameter wird später eingegangen.

In Abhängigkeit vom Einsatzgebiet können zahlreiche Parameter pro Artikel erforderlich sein. Häufig (eigentlich i.d.R.) gleichen die sich jedoch wiederum für ganze Artikelbereiche.

Für individuelle Fragestellungen können individualisierte Stammdaten erforderlich werden. Hierauf wird abschließend eingegangen.

Sowohl einem Artikelstamm, wie auch einem Artikel kann ein [Bild](./zuordnung_von_bildern.md) zugeordnet werden, auf das zum Beispiel per Formularposition zur Darstellung zugegriffen werden kann. Ist dabei einem Artikel ein Bild zugeordnet, so wird dieses, ansonsten das gegebenenfalls dem zugehörigen Artikelstamm zugeordnete Bild dargestellt.
