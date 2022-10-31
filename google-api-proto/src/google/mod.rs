#[cfg(
    any(
        feature = "google-actions-sdk-v2",
        feature = "google-actions-sdk-v2-conversation",
        feature = "google-actions-sdk-v2-interactionmodel",
        feature = "google-actions-sdk-v2-interactionmodel-prompt",
        feature = "google-actions-sdk-v2-interactionmodel-type",
        feature = "google-actions-type",
    )
)]
pub mod actions;
#[cfg(
    any(
        feature = "google-ads-admob-v1",
        feature = "google-ads-googleads-v10-common",
        feature = "google-ads-googleads-v10-enums",
        feature = "google-ads-googleads-v10-errors",
        feature = "google-ads-googleads-v10-resources",
        feature = "google-ads-googleads-v10-services",
        feature = "google-ads-googleads-v11-common",
        feature = "google-ads-googleads-v11-enums",
        feature = "google-ads-googleads-v11-errors",
        feature = "google-ads-googleads-v11-resources",
        feature = "google-ads-googleads-v11-services",
        feature = "google-ads-googleads-v12-common",
        feature = "google-ads-googleads-v12-enums",
        feature = "google-ads-googleads-v12-errors",
        feature = "google-ads-googleads-v12-resources",
        feature = "google-ads-googleads-v12-services",
        feature = "google-ads-googleads-v9-common",
        feature = "google-ads-googleads-v9-enums",
        feature = "google-ads-googleads-v9-errors",
        feature = "google-ads-googleads-v9-resources",
        feature = "google-ads-googleads-v9-services",
        feature = "google-ads-searchads360-v0-common",
        feature = "google-ads-searchads360-v0-enums",
        feature = "google-ads-searchads360-v0-resources",
        feature = "google-ads-searchads360-v0-services",
    )
)]
pub mod ads;
#[cfg(
    any(
        feature = "google-analytics-admin-v1alpha",
        feature = "google-analytics-admin-v1beta",
        feature = "google-analytics-data-v1alpha",
        feature = "google-analytics-data-v1beta",
    )
)]
pub mod analytics;
#[cfg(
    any(
        feature = "google-api",
        feature = "google-api-apikeys-v2",
        feature = "google-api-expr-conformance-v1alpha1",
        feature = "google-api-expr-v1alpha1",
        feature = "google-api-expr-v1beta1",
        feature = "google-api-servicecontrol-v1",
        feature = "google-api-servicecontrol-v2",
        feature = "google-api-servicemanagement-v1",
        feature = "google-api-serviceusage-v1",
        feature = "google-api-serviceusage-v1beta1",
    )
)]
pub mod api;
#[cfg(
    any(
        feature = "google-appengine-legacy",
        feature = "google-appengine-logging-v1",
        feature = "google-appengine-v1",
        feature = "google-appengine-v1beta",
    )
)]
pub mod appengine;
#[cfg(
    any(
        feature = "google-apps-alertcenter-v1beta1",
        feature = "google-apps-drive-activity-v2",
        feature = "google-apps-script-type",
        feature = "google-apps-script-type-calendar",
        feature = "google-apps-script-type-docs",
        feature = "google-apps-script-type-drive",
        feature = "google-apps-script-type-gmail",
        feature = "google-apps-script-type-sheets",
        feature = "google-apps-script-type-slides",
    )
)]
pub mod apps;
#[cfg(any(feature = "google-area120-tables-v1alpha1"))]
pub mod area120;
#[cfg(
    any(
        feature = "google-assistant-embedded-v1alpha1",
        feature = "google-assistant-embedded-v1alpha2",
    )
)]
pub mod assistant;
#[cfg(any(feature = "google-bigtable-admin-v2", feature = "google-bigtable-v2"))]
pub mod bigtable;
#[cfg(any(feature = "google-bytestream"))]
pub mod bytestream;
#[cfg(any(feature = "google-chat-logging-v1"))]
pub mod chat;
#[cfg(
    any(
        feature = "google-chromeos-moblab-v1beta1",
        feature = "google-chromeos-uidetection-v1",
    )
)]
pub mod chromeos;
#[cfg(
    any(
        feature = "google-cloud",
        feature = "google-cloud-accessapproval-v1",
        feature = "google-cloud-aiplatform-logging",
        feature = "google-cloud-aiplatform-v1",
        feature = "google-cloud-aiplatform-v1-schema-predict-instance",
        feature = "google-cloud-aiplatform-v1-schema-predict-params",
        feature = "google-cloud-aiplatform-v1-schema-predict-prediction",
        feature = "google-cloud-aiplatform-v1-schema-trainingjob-definition",
        feature = "google-cloud-aiplatform-v1beta1",
        feature = "google-cloud-aiplatform-v1beta1-schema",
        feature = "google-cloud-aiplatform-v1beta1-schema-predict-instance",
        feature = "google-cloud-aiplatform-v1beta1-schema-predict-params",
        feature = "google-cloud-aiplatform-v1beta1-schema-predict-prediction",
        feature = "google-cloud-aiplatform-v1beta1-schema-trainingjob-definition",
        feature = "google-cloud-apigateway-v1",
        feature = "google-cloud-apigeeconnect-v1",
        feature = "google-cloud-apigeeregistry-v1",
        feature = "google-cloud-asset-v1",
        feature = "google-cloud-asset-v1p1beta1",
        feature = "google-cloud-asset-v1p2beta1",
        feature = "google-cloud-asset-v1p5beta1",
        feature = "google-cloud-asset-v1p7beta1",
        feature = "google-cloud-assuredworkloads-regulatoryintercept-logging-v1",
        feature = "google-cloud-assuredworkloads-v1",
        feature = "google-cloud-assuredworkloads-v1beta1",
        feature = "google-cloud-audit",
        feature = "google-cloud-automl-v1",
        feature = "google-cloud-automl-v1beta1",
        feature = "google-cloud-baremetalsolution-v2",
        feature = "google-cloud-batch-v1",
        feature = "google-cloud-batch-v1alpha",
        feature = "google-cloud-beyondcorp-appconnections-v1",
        feature = "google-cloud-beyondcorp-appconnectors-v1",
        feature = "google-cloud-beyondcorp-appgateways-v1",
        feature = "google-cloud-beyondcorp-clientconnectorservices-v1",
        feature = "google-cloud-beyondcorp-clientgateways-v1",
        feature = "google-cloud-bigquery-analyticshub-v1",
        feature = "google-cloud-bigquery-connection-v1",
        feature = "google-cloud-bigquery-connection-v1beta1",
        feature = "google-cloud-bigquery-dataexchange-v1beta1",
        feature = "google-cloud-bigquery-datapolicies-v1beta1",
        feature = "google-cloud-bigquery-datatransfer-v1",
        feature = "google-cloud-bigquery-logging-v1",
        feature = "google-cloud-bigquery-migration-v2",
        feature = "google-cloud-bigquery-migration-v2alpha",
        feature = "google-cloud-bigquery-reservation-v1",
        feature = "google-cloud-bigquery-reservation-v1beta1",
        feature = "google-cloud-bigquery-storage-v1",
        feature = "google-cloud-bigquery-storage-v1beta1",
        feature = "google-cloud-bigquery-storage-v1beta2",
        feature = "google-cloud-bigquery-v2",
        feature = "google-cloud-billing-budgets-v1",
        feature = "google-cloud-billing-budgets-v1beta1",
        feature = "google-cloud-billing-v1",
        feature = "google-cloud-binaryauthorization-v1",
        feature = "google-cloud-binaryauthorization-v1beta1",
        feature = "google-cloud-certificatemanager-logging-v1",
        feature = "google-cloud-certificatemanager-v1",
        feature = "google-cloud-channel-v1",
        feature = "google-cloud-clouddms-logging-v1",
        feature = "google-cloud-clouddms-v1",
        feature = "google-cloud-cloudsetup-logging-v1",
        feature = "google-cloud-commerce-consumer-procurement-v1alpha1",
        feature = "google-cloud-common",
        feature = "google-cloud-compute-v1",
        feature = "google-cloud-compute-v1small",
        feature = "google-cloud-connectors-v1",
        feature = "google-cloud-contactcenterinsights-v1",
        feature = "google-cloud-contentwarehouse-v1",
        feature = "google-cloud-datacatalog-v1",
        feature = "google-cloud-datacatalog-v1beta1",
        feature = "google-cloud-dataform-v1alpha2",
        feature = "google-cloud-dataform-v1beta1",
        feature = "google-cloud-datafusion-v1",
        feature = "google-cloud-datafusion-v1beta1",
        feature = "google-cloud-datalabeling-v1beta1",
        feature = "google-cloud-dataplex-v1",
        feature = "google-cloud-dataproc-logging",
        feature = "google-cloud-dataproc-v1",
        feature = "google-cloud-dataqna-v1alpha",
        feature = "google-cloud-datastream-v1",
        feature = "google-cloud-datastream-v1alpha1",
        feature = "google-cloud-deploy-v1",
        feature = "google-cloud-dialogflow-cx-v3",
        feature = "google-cloud-dialogflow-cx-v3beta1",
        feature = "google-cloud-dialogflow-v2",
        feature = "google-cloud-dialogflow-v2beta1",
        feature = "google-cloud-discoveryengine-v1beta",
        feature = "google-cloud-documentai-v1",
        feature = "google-cloud-documentai-v1beta1",
        feature = "google-cloud-documentai-v1beta2",
        feature = "google-cloud-documentai-v1beta3",
        feature = "google-cloud-domains-v1",
        feature = "google-cloud-domains-v1alpha2",
        feature = "google-cloud-domains-v1beta1",
        feature = "google-cloud-edgecontainer-v1",
        feature = "google-cloud-enterpriseknowledgegraph-v1",
        feature = "google-cloud-essentialcontacts-v1",
        feature = "google-cloud-eventarc-publishing-v1",
        feature = "google-cloud-eventarc-v1",
        feature = "google-cloud-filestore-v1",
        feature = "google-cloud-filestore-v1beta1",
        feature = "google-cloud-functions-v1",
        feature = "google-cloud-functions-v2",
        feature = "google-cloud-functions-v2alpha",
        feature = "google-cloud-functions-v2beta",
        feature = "google-cloud-gaming-v1",
        feature = "google-cloud-gaming-v1beta",
        feature = "google-cloud-gkebackup-logging-v1",
        feature = "google-cloud-gkebackup-v1",
        feature = "google-cloud-gkeconnect-gateway-v1",
        feature = "google-cloud-gkeconnect-gateway-v1alpha1",
        feature = "google-cloud-gkeconnect-gateway-v1beta1",
        feature = "google-cloud-gkehub-cloudauditlogging-v1alpha",
        feature = "google-cloud-gkehub-configmanagement-v1",
        feature = "google-cloud-gkehub-configmanagement-v1alpha",
        feature = "google-cloud-gkehub-configmanagement-v1beta",
        feature = "google-cloud-gkehub-metering-v1alpha",
        feature = "google-cloud-gkehub-metering-v1beta",
        feature = "google-cloud-gkehub-multiclusteringress-v1",
        feature = "google-cloud-gkehub-multiclusteringress-v1alpha",
        feature = "google-cloud-gkehub-multiclusteringress-v1beta",
        feature = "google-cloud-gkehub-servicemesh-v1alpha",
        feature = "google-cloud-gkehub-v1",
        feature = "google-cloud-gkehub-v1alpha",
        feature = "google-cloud-gkehub-v1alpha2",
        feature = "google-cloud-gkehub-v1beta",
        feature = "google-cloud-gkehub-v1beta1",
        feature = "google-cloud-gkemulticloud-v1",
        feature = "google-cloud-gsuiteaddons-logging-v1",
        feature = "google-cloud-gsuiteaddons-v1",
        feature = "google-cloud-healthcare-logging",
        feature = "google-cloud-iap-v1",
        feature = "google-cloud-iap-v1beta1",
        feature = "google-cloud-identitytoolkit-logging",
        feature = "google-cloud-identitytoolkit-v2",
        feature = "google-cloud-ids-logging-v1",
        feature = "google-cloud-ids-v1",
        feature = "google-cloud-integrations-v1alpha",
        feature = "google-cloud-iot-v1",
        feature = "google-cloud-kms-v1",
        feature = "google-cloud-language-v1",
        feature = "google-cloud-language-v1beta1",
        feature = "google-cloud-language-v1beta2",
        feature = "google-cloud-lifesciences-v2beta",
        feature = "google-cloud-location",
        feature = "google-cloud-managedidentities-v1",
        feature = "google-cloud-managedidentities-v1beta1",
        feature = "google-cloud-mediatranslation-v1alpha1",
        feature = "google-cloud-mediatranslation-v1beta1",
        feature = "google-cloud-memcache-v1",
        feature = "google-cloud-memcache-v1beta2",
        feature = "google-cloud-metastore-logging-v1",
        feature = "google-cloud-metastore-v1",
        feature = "google-cloud-metastore-v1alpha",
        feature = "google-cloud-metastore-v1beta",
        feature = "google-cloud-networkanalyzer-logging-v1",
        feature = "google-cloud-networkconnectivity-v1",
        feature = "google-cloud-networkconnectivity-v1alpha1",
        feature = "google-cloud-networkmanagement-v1",
        feature = "google-cloud-networkmanagement-v1beta1",
        feature = "google-cloud-networksecurity-v1",
        feature = "google-cloud-networksecurity-v1beta1",
        feature = "google-cloud-networkservices-v1",
        feature = "google-cloud-networkservices-v1beta1",
        feature = "google-cloud-notebooks-logging-v1",
        feature = "google-cloud-notebooks-v1",
        feature = "google-cloud-notebooks-v1beta1",
        feature = "google-cloud-optimization-v1",
        feature = "google-cloud-orchestration-airflow-service-v1",
        feature = "google-cloud-orchestration-airflow-service-v1beta1",
        feature = "google-cloud-orgpolicy-v1",
        feature = "google-cloud-orgpolicy-v2",
        feature = "google-cloud-osconfig-agentendpoint-v1",
        feature = "google-cloud-osconfig-agentendpoint-v1beta",
        feature = "google-cloud-osconfig-v1",
        feature = "google-cloud-osconfig-v1alpha",
        feature = "google-cloud-osconfig-v1beta",
        feature = "google-cloud-oslogin-common",
        feature = "google-cloud-oslogin-v1",
        feature = "google-cloud-oslogin-v1alpha",
        feature = "google-cloud-oslogin-v1beta",
        feature = "google-cloud-paymentgateway-issuerswitch-v1",
        feature = "google-cloud-phishingprotection-v1beta1",
        feature = "google-cloud-policytroubleshooter-v1",
        feature = "google-cloud-privatecatalog-v1beta1",
        feature = "google-cloud-pubsublite-v1",
        feature = "google-cloud-recaptchaenterprise-v1",
        feature = "google-cloud-recaptchaenterprise-v1beta1",
        feature = "google-cloud-recommendationengine-v1beta1",
        feature = "google-cloud-recommender-logging-v1",
        feature = "google-cloud-recommender-logging-v1beta1",
        feature = "google-cloud-recommender-v1",
        feature = "google-cloud-recommender-v1beta1",
        feature = "google-cloud-redis-v1",
        feature = "google-cloud-redis-v1beta1",
        feature = "google-cloud-resourcemanager-v2",
        feature = "google-cloud-resourcemanager-v3",
        feature = "google-cloud-resourcesettings-v1",
        feature = "google-cloud-retail-logging",
        feature = "google-cloud-retail-v2",
        feature = "google-cloud-retail-v2alpha",
        feature = "google-cloud-retail-v2beta",
        feature = "google-cloud-run-v2",
        feature = "google-cloud-runtimeconfig-v1beta1",
        feature = "google-cloud-saasaccelerator-management-logs-v1",
        feature = "google-cloud-scheduler-v1",
        feature = "google-cloud-scheduler-v1beta1",
        feature = "google-cloud-secretmanager-logging-v1",
        feature = "google-cloud-secretmanager-v1",
        feature = "google-cloud-secrets-v1beta1",
        feature = "google-cloud-security-privateca-v1",
        feature = "google-cloud-security-privateca-v1beta1",
        feature = "google-cloud-security-publicca-v1beta1",
        feature = "google-cloud-securitycenter-settings-v1beta1",
        feature = "google-cloud-securitycenter-v1",
        feature = "google-cloud-securitycenter-v1beta1",
        feature = "google-cloud-securitycenter-v1p1beta1",
        feature = "google-cloud-sensitiveaction-logging-v1",
        feature = "google-cloud-servicedirectory-v1",
        feature = "google-cloud-servicedirectory-v1beta1",
        feature = "google-cloud-shell-v1",
        feature = "google-cloud-speech-v1",
        feature = "google-cloud-speech-v1p1beta1",
        feature = "google-cloud-speech-v2",
        feature = "google-cloud-sql-v1",
        feature = "google-cloud-sql-v1beta4",
        feature = "google-cloud-storageinsights-v1",
        feature = "google-cloud-stream-logging-v1",
        feature = "google-cloud-support-common",
        feature = "google-cloud-support-v1alpha1",
        feature = "google-cloud-talent-v4",
        feature = "google-cloud-talent-v4beta1",
        feature = "google-cloud-tasks-v2",
        feature = "google-cloud-tasks-v2beta2",
        feature = "google-cloud-tasks-v2beta3",
        feature = "google-cloud-texttospeech-v1",
        feature = "google-cloud-texttospeech-v1beta1",
        feature = "google-cloud-timeseriesinsights-v1",
        feature = "google-cloud-tpu-v1",
        feature = "google-cloud-tpu-v2alpha1",
        feature = "google-cloud-translation-v3",
        feature = "google-cloud-translation-v3beta1",
        feature = "google-cloud-video-livestream-logging-v1",
        feature = "google-cloud-video-livestream-v1",
        feature = "google-cloud-video-stitcher-v1",
        feature = "google-cloud-video-transcoder-v1",
        feature = "google-cloud-videointelligence-v1",
        feature = "google-cloud-videointelligence-v1beta2",
        feature = "google-cloud-videointelligence-v1p1beta1",
        feature = "google-cloud-videointelligence-v1p2beta1",
        feature = "google-cloud-videointelligence-v1p3beta1",
        feature = "google-cloud-vision-v1",
        feature = "google-cloud-vision-v1p1beta1",
        feature = "google-cloud-vision-v1p2beta1",
        feature = "google-cloud-vision-v1p3beta1",
        feature = "google-cloud-vision-v1p4beta1",
        feature = "google-cloud-visionai-v1alpha1",
        feature = "google-cloud-vmmigration-v1",
        feature = "google-cloud-vpcaccess-v1",
        feature = "google-cloud-webrisk-v1",
        feature = "google-cloud-webrisk-v1beta1",
        feature = "google-cloud-websecurityscanner-v1",
        feature = "google-cloud-websecurityscanner-v1alpha",
        feature = "google-cloud-websecurityscanner-v1beta",
        feature = "google-cloud-workflows-executions-v1",
        feature = "google-cloud-workflows-executions-v1beta",
        feature = "google-cloud-workflows-type",
        feature = "google-cloud-workflows-v1",
        feature = "google-cloud-workflows-v1beta",
    )
)]
pub mod cloud;
#[cfg(
    any(
        feature = "google-container-v1",
        feature = "google-container-v1alpha1",
        feature = "google-container-v1beta1",
    )
)]
pub mod container;
#[cfg(any(feature = "google-dataflow-v1beta3"))]
pub mod dataflow;
#[cfg(
    any(
        feature = "google-datastore-admin-v1",
        feature = "google-datastore-admin-v1beta1",
        feature = "google-datastore-v1",
        feature = "google-datastore-v1beta3",
    )
)]
pub mod datastore;
#[cfg(
    any(
        feature = "google-devtools-artifactregistry-v1",
        feature = "google-devtools-artifactregistry-v1beta2",
        feature = "google-devtools-build-v1",
        feature = "google-devtools-cloudbuild-v1",
        feature = "google-devtools-clouddebugger-v2",
        feature = "google-devtools-clouderrorreporting-v1beta1",
        feature = "google-devtools-cloudprofiler-v2",
        feature = "google-devtools-cloudtrace-v1",
        feature = "google-devtools-cloudtrace-v2",
        feature = "google-devtools-containeranalysis-v1",
        feature = "google-devtools-containeranalysis-v1beta1",
        feature = "google-devtools-remoteworkers-v1test2",
        feature = "google-devtools-resultstore-v2",
        feature = "google-devtools-source-v1",
        feature = "google-devtools-sourcerepo-v1",
        feature = "google-devtools-testing-v1",
    )
)]
pub mod devtools;
#[cfg(
    any(
        feature = "google-example-endpointsapis-v1",
        feature = "google-example-library-v1",
    )
)]
pub mod example;
#[cfg(any(feature = "google-firebase-fcm-connection-v1alpha1"))]
pub mod firebase;
#[cfg(
    any(
        feature = "google-firestore-admin-v1",
        feature = "google-firestore-admin-v1beta1",
        feature = "google-firestore-admin-v1beta2",
        feature = "google-firestore-bundle",
        feature = "google-firestore-v1",
        feature = "google-firestore-v1beta1",
    )
)]
pub mod firestore;
#[cfg(any(feature = "google-gapic-metadata"))]
pub mod gapic;
#[cfg(any(feature = "google-genomics-v1", feature = "google-genomics-v1alpha2"))]
pub mod genomics;
#[cfg(any(feature = "google-geo-type"))]
pub mod geo;
#[cfg(any(feature = "google-home-enterprise-sdm-v1", feature = "google-home-graph-v1"))]
pub mod home;
#[cfg(
    any(
        feature = "google-iam-admin-v1",
        feature = "google-iam-credentials-v1",
        feature = "google-iam-v1",
        feature = "google-iam-v1-logging",
        feature = "google-iam-v1beta",
        feature = "google-iam-v2",
        feature = "google-iam-v2beta",
    )
)]
pub mod iam;
#[cfg(
    any(
        feature = "google-identity-accesscontextmanager-type",
        feature = "google-identity-accesscontextmanager-v1",
    )
)]
pub mod identity;
#[cfg(any(feature = "google-logging-type", feature = "google-logging-v2"))]
pub mod logging;
#[cfg(any(feature = "google-longrunning"))]
pub mod longrunning;
#[cfg(
    any(
        feature = "google-maps-addressvalidation-v1",
        feature = "google-maps-mapsplatformdatasets-v1alpha",
        feature = "google-maps-playablelocations-v3",
        feature = "google-maps-playablelocations-v3-sample",
        feature = "google-maps-regionlookup-v1alpha",
        feature = "google-maps-roads-v1op",
        feature = "google-maps-routes-v1",
        feature = "google-maps-routes-v1alpha",
        feature = "google-maps-routing-v2",
        feature = "google-maps-unity",
    )
)]
pub mod maps;
#[cfg(
    any(
        feature = "google-monitoring-dashboard-v1",
        feature = "google-monitoring-metricsscope-v1",
        feature = "google-monitoring-v3",
    )
)]
pub mod monitoring;
#[cfg(any(feature = "google-networking-trafficdirector-type"))]
pub mod networking;
#[cfg(any(feature = "google-partner-aistreams-v1alpha1"))]
pub mod partner;
#[cfg(any(feature = "google-privacy-dlp-v2"))]
pub mod privacy;
#[cfg(any(feature = "google-pubsub-v1", feature = "google-pubsub-v1beta2"))]
pub mod pubsub;
#[cfg(any(feature = "google-type"))]
pub mod r#type;
#[cfg(any(feature = "google-rpc", feature = "google-rpc-context"))]
pub mod rpc;
#[cfg(any(feature = "google-search-partnerdataingestion-logging-v1"))]
pub mod search;
#[cfg(
    any(
        feature = "google-spanner-admin-database-v1",
        feature = "google-spanner-admin-instance-v1",
        feature = "google-spanner-v1",
    )
)]
pub mod spanner;
#[cfg(any(feature = "google-storage-v1", feature = "google-storage-v2"))]
pub mod storage;
#[cfg(
    any(
        feature = "google-storagetransfer-logging",
        feature = "google-storagetransfer-v1",
    )
)]
pub mod storagetransfer;
#[cfg(any(feature = "google-streetview-publish-v1"))]
pub mod streetview;
#[cfg(any(feature = "google-watcher-v1"))]
pub mod watcher;
