# Automatische Prüfung und Abholung

<!-- source: https://amic.de/hilfe/automatischeprfungundabholung.htm -->

Mit Hilfe eines regelmäßig aufgerufenen Events ist es möglich die Statusprüfung und die Abholung bereitgestellter Ausfuhrbegleit-Dokumente und Ausgangsvermerk-Dokumente zu gewährleisten.

Sollen dabei Kontrollmitteilungen über bereitgestellte oder abgelehnte Zollausfuhranmeldungen versendet werden, so muss im Versandprofilstamm (ehem. Verpostungsstamm) ein Eintrag für den Typ „*Infomails Zollanwicklung*“ erstellt werden.

Die Zieladressen für diese beiden Ereignisse sind im Mandantenstamm auf der Registerkarte Zollausfuhr einzutragen.

Ein regelmäßiger Aufruf der Prozedur *AMIC_GET_ZOLL_STATUS* oder einer analogen Prozedur im Rahmen eines Events sorgt für die gewünschte Abarbeitung.
