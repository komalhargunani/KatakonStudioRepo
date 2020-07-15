<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_LRS Connect                 Console-po_8f0059</name>
   <tag></tag>
   <elementGuidId>5d51e548-af63-4ec2-a29d-825b08ed1374</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//*/text()[normalize-space(.)='']/parent::*</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>lang</name>
      <type>Main</type>
      <value>en</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
	
	
	LRS Connect
    
    
        // Console-polyfill. MIT license.
        // https://github.com/paulmillr/console-polyfill
        // Make it safe to do console.log() always.
        (function(con) {
            'use strict';
            var prop, method;
            var empty = {};
            var dummy = function() {};
            var properties = 'memory'.split(',');
            var methods = ('assert,clear,count,debug,dir,dirxml,error,exception,group,' +
                    'groupCollapsed,groupEnd,info,log,markTimeline,profile,profiles,profileEnd,' +
                    'show,table,time,timeEnd,timeline,timelineEnd,timeStamp,trace,warn').split(',');
            while (prop = properties.pop()) con[prop] = con[prop] || empty;
            while (method = methods.pop()) con[method] = con[method] || dummy;
        })(this.console = this.console || {}); // Using `this` for web workers.

        (function(i,s,o,g,r,a,m){i['GoogleAnalyticsObject']=r;i[r]=i[r]||function(){
            (i[r].q=i[r].q||[]).push(arguments)},i[r].l=1*new Date();a=s.createElement(o),
                m=s.getElementsByTagName(o)[0];a.async=1;a.src=g;m.parentNode.insertBefore(a,m)
        })(window,document,'script','//www.google-analytics.com/analytics.js','ga');

        ga('create', 'UA-420934-15','auto');
    
    
    
        enableLogs = false;
    
    

    
#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}

    
        
        
        
        
        

        
    
    
        var lrsCacheFlushVersion=&quot;ca779536df4de05b3887ca18cb8cd2e6&quot;;
        var fileref=document.createElement('script')
        fileref.setAttribute(&quot;type&quot;,&quot;text/javascript&quot;)
        fileref.setAttribute(&quot;src&quot;, 'js/lib/require/require.js');
        fileref.setAttribute('data-main','js/main.js?v='+lrsCacheFlushVersion);
        document.getElementsByTagName(&quot;head&quot;)[0].appendChild(fileref);
    


/html[1]</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>//*/text()[normalize-space(.)='']/parent::*</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//html</value>
   </webElementXpaths>
</WebElementEntity>
