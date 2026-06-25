# Beispiel Betrag in Worten im Formular

<!-- source: https://amic.de/hilfe/beispielbetraginwortenimformul.htm -->

// Priv. Prozedur p_BetragInPolnisch --- BT 10.08.2005

//

// Beschreibung:

//

//

//

CREATE Function p_BetragInPolnisch (  
 in in_ZiffernVorkomma char(15),  
 in in_ZiffernNachkomma char(15),  
 in in_Vorzeichen integer,  
 in in_Dezimalstellen integer,  
 in in_Betrag numeric(15,6)  
 )  
 RETURNS char(500)  
BEGIN  
DECLARE text char(200);  
declare hilf_text char(500);  
declare hunderttausender char(1);  
declare zehntausender char(1);  
declare tausender char(1);  
declare hunderter char(3);  
declare zehner char(2);  
declare einer char(1);  
declare nachkomma char(500);  
declare local temporary table zt  
 ( zt char(40),  
 z integer  
 ) on commit delete rows ;  
 insert into zt ( z, zt ) values ( 0, '' );  
 insert into zt ( z, zt ) values ( 1, 'jeden' );  
 insert into zt ( z, zt ) values ( 2, 'dwa' );  
 insert into zt ( z, zt ) values ( 3, 'trzy' );  
 insert into zt ( z, zt ) values ( 4, 'cztery' );  
 insert into zt ( z, zt ) values ( 5, 'plec' );  
 insert into zt ( z, zt ) values ( 6, 'szesc' );  
 insert into zt ( z, zt ) values ( 7, 'siedem' );  
 insert into zt ( z, zt ) values ( 8, 'osiem' );  
 insert into zt ( z, zt ) values ( 9, 'dziewiec' );  
 insert into zt ( z, zt ) values ( 10, 'dziesiec' );  
 insert into zt ( z, zt ) values ( 11, 'jedenascie' );  
 insert into zt ( z, zt ) values ( 12, 'dwanascie' );  
 insert into zt ( z, zt ) values ( 13, 'trzynascie' );  
 insert into zt ( z, zt ) values ( 14, 'czternascie' );  
 insert into zt ( z, zt ) values ( 15, 'pietnascie' );  
 insert into zt ( z, zt ) values ( 16, 'szesnascie' );  
 insert into zt ( z, zt ) values ( 17, 'siedemnascie' );  
 insert into zt ( z, zt ) values ( 18, 'osiemnascie' );  
 insert into zt ( z, zt ) values ( 19, 'dziewietnascie' );  
 insert into zt ( z, zt ) values ( 20, 'dwadziescia' );  
 insert into zt ( z, zt ) values ( 30, 'trzydziesci' );  
 insert into zt ( z, zt ) values ( 40, 'czterdziesci' );  
 insert into zt ( z, zt ) values ( 50, 'piecdziesiat' );  
 insert into zt ( z, zt ) values ( 60, 'szescdziesiat' );  
 insert into zt ( z, zt ) values ( 70, 'siedemdziesiat' );  
 insert into zt ( z, zt ) values ( 80, 'osiemdzieslat' );  
 insert into zt ( z, zt ) values ( 90, 'dziewiecdziesiat' );  
 insert into zt ( z, zt ) values ( 100, 'sto' );  
 insert into zt ( z, zt ) values ( 200, 'dwiescie' );  
 insert into zt ( z, zt ) values ( 300, 'trzysta' );  
 insert into zt ( z, zt ) values ( 400, 'czterysta' );  
 insert into zt ( z, zt ) values ( 500, 'piecset' );  
 insert into zt ( z, zt ) values ( 600, 'szescset' );  
 insert into zt ( z, zt ) values ( 700, 'siedemset' );  
 insert into zt ( z, zt ) values ( 800, 'osiemset' );  
 insert into zt ( z, zt ) values ( 900, 'dziewiecset' );  
 insert into zt ( z, zt ) values ( 1000, 'tysiak' );  
 insert into zt ( z, zt ) values ( 1000000, 'milion' );  
 insert into zt ( z, zt ) values ( 2000000, 'miliony' );  
\-- insert into zt ( z, zt ) values ( 0, '' );  
 set text = NULL;  
 select zt into text from zt where z = in_ZiffernVorkomma ;  
 if text is NULL then  
 if length ( in_ZiffernVorkomma ) = 2 then  
 set zehner = substring( in_ZiffernVorkomma, 1, 1 )&#124;&#124;'0';  
 set einer = substring( in_ZiffernVorkomma, 2, 1 );  
 select zt&#124;&#124;' '&#124;&#124;  
 (select zt from zt where z = einer) into text from zt where z = zehner ;  
 end if;  
 if length ( in_ZiffernVorkomma ) = 3 then  
 set hunderter = substring( in_ZiffernVorkomma, 1, 1 )&#124;&#124;'00';  
 set zehner = substring( in_ZiffernVorkomma, 2, 1 )&#124;&#124;'0';  
 set einer = substring( in_ZiffernVorkomma, 3, 1 );  
 select zt&#124;&#124;' '&#124;&#124;  
 (select zt from zt where z = zehner)&#124;&#124;' '&#124;&#124;  
 (select zt from zt where z = einer)  
 into text from zt where z = hunderter ;  
 end if;  
 if length ( in_ZiffernVorkomma ) = 4 then  
 set tausender = substring( in_ZiffernVorkomma, 1, 1 );  
 set hunderter = substring( in_ZiffernVorkomma, 2, 1 )&#124;&#124;'00';  
 set zehner = substring( in_ZiffernVorkomma, 3, 1 )&#124;&#124;'0';  
 set einer = substring( in_ZiffernVorkomma, 4, 1 );  
 select  
 (select zt from zt where z = tausender)&#124;&#124;' '&#124;&#124;  
 zt&#124;&#124;' '&#124;&#124;  
 (select zt from zt where z = zehner)&#124;&#124;' '&#124;&#124;  
 (select zt from zt where z = einer)  
 into text from zt where z = 1000 ;  
 end if;  
 if length ( in_ZiffernVorkomma ) = 5 then  
 set hilf_text = NULL;  
 select zt into hilf_text from zt where z = substring(in_ZiffernVorkomma,1,2) ;  
 if ( hilf_text is NULL ) then  
 set zehner = substring( in_ZiffernVorkomma, 1, 1 )&#124;&#124;'0';  
 set einer = substring( in_ZiffernVorkomma, 2, 1 );  
 select zt&#124;&#124;' '&#124;&#124;  
 (select zt from zt where z = einer) into hilf_text from zt where z = zehner ;  
 end if;  
 set hunderter = substring( in_ZiffernVorkomma, 3, 1 )&#124;&#124;'00';  
 set zehner = substring( in_ZiffernVorkomma, 4, 1 )&#124;&#124;'0';  
 set einer = substring( in_ZiffernVorkomma, 5, 1 );  
 select zt&#124;&#124;' '&#124;&#124;  
 (select zt from zt where z = zehner)&#124;&#124;' '&#124;&#124;  
 (select zt from zt where z = einer)  
 into text from zt where z = hunderter ;  
 select hilf_text&#124;&#124;' '&#124;&#124;zt&#124;&#124;' '&#124;&#124;text into text from zt where z = 1000;  
 end if;  
 if length ( in_ZiffernVorkomma ) = 6 then  
 set hilf_text = NULL;  
 set hunderter = substring( in_ZiffernVorkomma, 1, 1 )&#124;&#124;'00';  
 set zehner = substring( in_ZiffernVorkomma, 2, 1 )&#124;&#124;'0';  
 set einer = substring( in_ZiffernVorkomma, 3, 1 );  
 select zt&#124;&#124;' '&#124;&#124;  
 (select zt from zt where z = zehner)&#124;&#124;' '&#124;&#124;  
 (select zt from zt where z = einer)  
 into hilf_text from zt where z = hunderter ;  
 set hunderter = substring( in_ZiffernVorkomma, 4, 1 )&#124;&#124;'00';  
 set zehner = substring( in_ZiffernVorkomma, 5, 1 )&#124;&#124;'0';  
 set einer = substring( in_ZiffernVorkomma, 6, 1 );  
 select zt&#124;&#124;' '&#124;&#124;  
 (select zt from zt where z = zehner)&#124;&#124;' '&#124;&#124;  
 (select zt from zt where z = einer)  
 into text from zt where z = hunderter ;  
 select hilf_text&#124;&#124;' '&#124;&#124;zt&#124;&#124;' '&#124;&#124;text into text from zt where z = 1000;  
 end if;  
 if length ( in_ZiffernVorkomma ) = 7 then  
 set hilf_text = NULL;  
 set hunderter = substring( in_ZiffernVorkomma, 2, 1 )&#124;&#124;'00';  
 set zehner = substring( in_ZiffernVorkomma, 3, 1 )&#124;&#124;'0';  
 set einer = substring( in_ZiffernVorkomma, 4, 1 );  
 select zt&#124;&#124;' '&#124;&#124;  
 (select zt from zt where z = zehner)&#124;&#124;' '&#124;&#124;  
 (select zt from zt where z = einer)  
 into hilf_text from zt where z = hunderter ;  
 set hunderter = substring( in_ZiffernVorkomma, 5, 1 )&#124;&#124;'00';  
 set zehner = substring( in_ZiffernVorkomma, 6, 1 )&#124;&#124;'0';  
 set einer = substring( in_ZiffernVorkomma, 7, 1 );  
 select zt&#124;&#124;' '&#124;&#124;  
 (select zt from zt where z = zehner)&#124;&#124;' '&#124;&#124;  
 (select zt from zt where z = einer)  
 into text from zt where z = hunderter ;  
 select (if substring(in_ziffernvorkomma,1,1)=1 then  
 (select zt from zt where z=1000000) else  
 (select (select zt from zt where z=substring(in_ziffernvorkomma,1,1))&#124;&#124;' '&#124;&#124;zt from zt where z>1000000) endif  
 )&#124;&#124;' '&#124;&#124;  
 hilf_text&#124;&#124;' '&#124;&#124;zt&#124;&#124;' '&#124;&#124;text into text from zt where z = 1000;  
 end if;  
 end if;  
 set zehner = substring( in_ZiffernNachkomma, 1, 1 )&#124;&#124;'0';  
 set einer = substring( in_ZiffernNachkomma, 2, 1 );  
 set nachkomma = NULL;  
 select zt into nachkomma from zt where z = in_ZiffernNachkomma ;  
 if nachkomma is NULL then  
 select zt&#124;&#124;' '&#124;&#124; (select zt from zt where z = einer) into nachkomma from zt where z = zehner ;  
 end if;  
 set text = text &#124;&#124; ' Zlotych ' &#124;&#124; isnull(nachkomma,'zero') &#124;&#124; ' Grosz';  
 RETURN text;  
END
