# Einrichtung von openTRANS für FiBu

<!-- source: https://amic.de/hilfe/_einrFiBuOT.htm -->

Zinsabrechnungen in der Finanzbuchhaltung können für Kunden, bei denen das openTRANS-Kennzeichen aktiviert ist, auch beim Drucken als openTRANS übertragen werden.

Die Einrichtung setzt die Einrichtung von openTRANS für Vorgänge voraus.

Steuerparameter

Administration > Steuerung > Steuerparameter anzeigen

• Der Steuerparameter [Steuerparameter 721 – openTRANS](../../../../firmenstamm/steuerparameter/lizenzen/opentrans_lizenz_spa_721.md) (Lizenz) muss aktiviert sein, um diese Option zu nutzen.

• Der Steuerparameter [840 - FiBu Zinsbelege mit openTRANS drucken](../../../../firmenstamm/steuerparameter/optionen_finanzwesen/fibu_zinsbelege_mit_opentrans_drucken_spa_840.md) muss aktiviert sein, um diese Option zu nutzen.

• Der Steuerparameter [841 - FRZ-Unterklasse für FiBu-Zinsbelege](../../../../firmenstamm/steuerparameter/optionen_finanzwesen/frz_unterklasse_fuer_fibu_zinsbelege_spa_841.md) (Default ist 0) muss festlegen, aus welcher Unterklasse der Rechnung (Vorgangsklasse700) der Export seine Einstellungen übernehmen soll. 

Sachkontenstamm

Finanzbuchhaltung > Stammdaten > Sachkonten

In der Finanzbuchhaltung gibt es verschiedene Sachkonten, auf die Teile der Abrechnung gebucht werden. Die Verwendung dafür kann z.B. sein:

• Soll-Zinsen

• Haben-Zinsen

• Zinsabschlagsteuer

• Solidarzuschlag

• Kirchensteuer

• Gebühren

Im [Sachkontenstamm](../../../../finanzbuchhaltung/stammdaten_der_fibu/sachkonten.md) können Sie auf der Registerkarte weitere Optionen eine Artikelnummer festlegen. Dieser „künstliche“ Artikel beschreibt die Art der Buchung in der Zinsabrechnung. Der Artikel darf keine Steuern, Zu-/Abschläge o.ä. haben.

FiBu-openTRANS-Optionen

In der Formularzuordnung für die Vorgangsklassen werden openTRANS-Export-Optionen eingegeben. Der Steuerparameter [841 - FRZ-Unterklasse für FiBu-Zinsbelege](../../../../firmenstamm/steuerparameter/optionen_finanzwesen/frz_unterklasse_fuer_fibu_zinsbelege_spa_841.md) legt fest, welche Unterklasse der Vorgangsklasse 700 (Rechnung) diese Einstellungen enthalten soll.

Zur Beschreibung der Felder lesen Sie dazu bitte den Abschnitt: Vorgangsabwicklung > Formularzuordnung [FRZ] > openTRANS.
