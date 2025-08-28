/**
 * @file:       Subscriber.h
 *
 * copyright:   Copyright (c) 2018 ZhenRong Technology, Inc. All rights reserved.
 */

#ifndef Subscriber_h__
#define Subscriber_h__

#include "SubscriberQos.h"
#include "SubscriberListener.h"
#include "StatusKindMask.h"
#include "DataReaderQos.h"
#include "TopicQos.h"
#include "SampleStateMask.h"
#include "ViewStateMask.h"
#include "InstanceStateMask.h"
#include "ReturnCode_t.h"
#include "DataReader.h"
#include "ZRDDSTypeSupport.h"

#ifdef __cplusplus
extern "C"
{
#endif

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Subscriber_get_qos( DDS_Subscriber* self, DDS_SubscriberQos* qoslist);
 *
 * @ingroup CSubscription
 *
 * @brief   ��ȡ�ö����ߵ�QoS���á�
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in,out]  qoslist ���ڲ��������ڱ��涩���ߵ�QoS���á�
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK :��ʾ��ȡ�ɹ���
 *          - #DDS_RETCODE_ERROR :��ʾʧ�ܣ�ԭ�����Ϊ����QoSʱ��������
 */

DCPSDLL DDS_ReturnCode_t DDS_Subscriber_get_qos(
    DDS_Subscriber* self,
    DDS_SubscriberQos* qoslist);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Subscriber_set_qos( DDS_Subscriber* self, const DDS_SubscriberQos* qoslist);
 *
 * @ingroup CSubscription
 *
 * @brief   �÷�������Ϊ���������õ�QoS��
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   qoslist ��ʾ�û���Ҫ���õ�QoS���á�
 *
 * @details ����ʹ������ֵ #DDS_SUBSCRIBER_QOS_DEFAULT ��ʾʹ�ô洢����������е�QoS���á�
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK :��ʾ��ȡ�ɹ���
 *          - #DDS_RETCODE_BAD_PARAMETER :��ʾ @e qoslist ������Ч��QoS���ã�
 *          - #DDS_RETCODE_INCONSISTENT :��ʾ @e qoslist ���в����ݵ�QoS���ã�
 *          - #DDS_RETCODE_IMMUTABLE_POLICY :��ʾ�û������޸�ʹ�ܺ󲻿ɱ��QoS���ã�
 *          - #DDS_RETCODE_ERROR :��ʾδ����Ĵ��󣬴�����ϸ��Ϣ�������־�У�
 */

DCPSDLL DDS_ReturnCode_t DDS_Subscriber_set_qos(
    DDS_Subscriber* self, 
    const DDS_SubscriberQos* qoslist);

/**
 * @fn  DCPSDLL DDS_SubscriberListener* DDS_Subscriber_get_listener( DDS_Subscriber* self);
 *
 * @ingroup CSubscription
 *
 * @brief   �÷�����ȡͨ�� #DDS_Subscriber_set_listener �������ߴ���ʱΪ���������õļ���������
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 *                  
 * @return  ��ǰ���ܵķ���ֵ��
 *          - NULL��ʾδ���ü�������
 *          - �ǿձ�ʾӦ��ͨ�� #DDS_Subscriber_set_listener �����ڴ���ʱ���õļ���������
 */

DCPSDLL DDS_SubscriberListener* DDS_Subscriber_get_listener(
    DDS_Subscriber* self);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Subscriber_set_listener( DDS_Subscriber* self, DDS_SubscriberListener* listener, DDS_StatusKindMask mask);
 *
 * @ingroup CSubscription
 *
 * @brief   ���øö����ߵļ�������
 *
 * @details  ������������ԭ�м�������������ÿն����ʾ���ԭ�����õļ�������
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in,out]  listener  Ϊ�ö��������õļ���������
 * @param   mask                ����Ӧ�ó������Ȥ��״̬��ֻ��Ӧ�ó������Ȥ��״̬�����仯ʱ���Ż�֪ͨӦ�ó���
 *
 * @return  ��ǰ���Ƿ��� #DDS_RETCODE_OK ��ʾ���óɹ���
 */

DCPSDLL DDS_ReturnCode_t DDS_Subscriber_set_listener(
    DDS_Subscriber* self,
    DDS_SubscriberListener* listener,
    DDS_StatusKindMask mask);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Subscriber_enable( DDS_Subscriber* self);
 *
 * @ingroup CSubscription
 *
 * @brief   �ֶ�ʹ�ܸ�ʵ�壬�μ�@ref entity-enable ��
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK ����ʾ��ȡ�ɹ���  
 *          - #DDS_RETCODE_PRECONDITION_NOT_MET ����ʾ�����ĸ�ʵ����δʹ��
 */

DCPSDLL DDS_ReturnCode_t DDS_Subscriber_enable(
    DDS_Subscriber* self);

/**
 * @fn  DCPSDLL DDS_DataReader* DDS_Subscriber_create_datareader( DDS_Subscriber* self, DDS_TopicDescription* topic, const DDS_DataReaderQos* qos, DDS_DataReaderListener* listener, DDS_StatusKindMask mask);
 *
 * @ingroup CSubscription
 *
 * @brief   �÷����ڶ������´���һ�����ݶ�����ʵ�壬�����ù��������⡢QoS�Լ���������
 *
 * @details �û�ʹ�ø����ݶ��ߴ����ڶ�ȡ/��ȡָ���������ݣ����ص����ݶ��߶���Ϊ���ݶ��߹������û�����������ص����ݶ��ߵĸ���ָ�룬
 *          �û���Ҫ������ֵ��̬ת��Ϊ�û��������͵����ݶ��߶��󣬾������μ� @ref subscription_example.c ��
 *          ��
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in,out]  topic     �����������������û����Թ��������Լ��������ݹ��˵����⣬
 *                              ���������������ڵ��ø÷���֮ǰ��ͬһ����������µ��� #DDS_DomainParticipant_create_topic
 *                               #DDS_DomainParticipant_create_contentfilteredtopic ��������������ĸ��ࣻ
 * @param   qos             ��ʾΪ�����ݶ������õ�QoS�� #DDS_DATAREADER_QOS_DEFAULT ����ʹ�ö������б����Ĭ�ϵ�QoS��
 * @param [in,out]  listener  Ϊ�ö��������õļ��������˲�������Ϊ�ա� ZRDDS����ӹܼ�����������ڴ�������û������ͷš�
 * @param   mask                ����Ӧ�ó������Ȥ��״̬��ֻ��Ӧ�ó������Ȥ��״̬�����仯ʱ���Ż�֪ͨӦ�ó���
 *
 * @return  ��NULL��ʾ�����ɹ��������ʾʧ�ܣ�ʧ�ܵ�ԭ�����Ϊ��
 *          - @e topic ������Ч���������
 *          - @e topic �ĸ�ʵ����ö����߲�����һ���������ʵ�壻
 *          - @e qos �к�����Ч��QoS���ߺ��в�һ�µ�QoS���ã�
 *          - �������ݴ����δ���������ϸ�μ���־��
 */

DCPSDLL DDS_DataReader* DDS_Subscriber_create_datareader(
    DDS_Subscriber* self, 
    DDS_TopicDescription* topic,
    const DDS_DataReaderQos* qos,
    DDS_DataReaderListener* listener,
    DDS_StatusKindMask mask);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Subscriber_delete_datareader( DDS_Subscriber* self, DDS_DataReader* reader);
 *
 * @ingroup CSubscription
 *
 * @brief   ɾ��ָ�������ݶ��ߡ�
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in,out]  reader    ָ����Ҫɾ�������ݶ��ߡ�
 *
 * @details ��ɾ�������ݶ���Ӧ����ɾ����������Ҫ������
 *          - ���ݶ��ߴ��������С���ʵ�塱��ȡ�����Ѿ�ȫ����ɾ����
 *          - ͨ�������ݶ��ߵ� @ref read-take ϵ�з��������û��Ŀռ��Ѿ�ȫ�����ճɹ���
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *          - #DDS_RETCODE_OK ��ɾ���ɹ���
 *          - #DDS_RETCODE_BAD_PARAMETER ������ָ�������ݶ��߲�����Ч�����ݶ��߶���
 *          - #DDS_RETCODE_PRECONDITION_NOT_MET ��
 *              - ����ָ�������ݶ��߲����ڱ���
 *              - ָ�������ݶ��߲�����ɾ��������
 */

DCPSDLL DDS_ReturnCode_t DDS_Subscriber_delete_datareader(
    DDS_Subscriber* self, 
    DDS_DataReader* reader);

/**
 * @fn  DCPSDLL DDS_DataReader* DDS_Subscriber_lookup_datareader( DDS_Subscriber* self, const DDS_Char* topicName);
 *
 * @ingroup CSubscription
 * @brief   �����������������ݶ��ߡ�
 *
 * @details ������ڶ���������������ݶ��ߣ��򷵻����ݶ��ߵ�ַ��С���Ǹ���
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   topicName  ��ѯ����������
 *
 * @return  ���ؿձ�ʾû���������������ݶ��ߣ����򷵻���Ӧ�����ݶ��ߡ�
 */

DCPSDLL DDS_DataReader* DDS_Subscriber_lookup_datareader(
    DDS_Subscriber* self, 
    const DDS_Char* topicName);

#ifdef _ZRDDS_INCLUDE_PRESENTATION_QOS

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Subscriber_begin_access( DDS_Subscriber* self);
 *
 * @ingroup CSubscription
 *
 * @brief   ���ݶ��߶˿���һ���Է��ʡ�
 *
 * @details  �˷���ֻ���ڶ����� DDS_PresentationQosPolicy::access_scope == #DDS_GROUP_PRESENTATION_QOS
 *           ʱ��Ч���÷���Ӧ�� #DDS_Subscriber_end_access �������ʹ�á�
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 *                  
 * @return  ��ǰ���Ƿ��� #DDS_RETCODE_UNSUPPORTED ��
 *
 * @warning �÷���δʵ�֡�
 */

DCPSDLL DDS_ReturnCode_t DDS_Subscriber_begin_access(
    DDS_Subscriber* self);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Subscriber_end_access( DDS_Subscriber* self);
 *
 * @ingroup CSubscription
 *
 * @brief   �� #DDS_Subscriber_begin_access ������Ӧ����ʾ���ݷ��ʽ�����
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * 
 * @return  ��ǰ���Ƿ��� #DDS_RETCODE_UNSUPPORTED ��
 *
 * @warning �÷���δʵ�֡�
 */

DCPSDLL DDS_ReturnCode_t DDS_Subscriber_end_access(
    DDS_Subscriber* self);

#endif /* _ZRDDS_INCLUDE_PRESENTATION_QOS */

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Subscriber_get_datareaders( DDS_Subscriber* self, DDS_DataReaderSeq* readers, DDS_SampleStateMask sampleStates, DDS_ViewStateMask viewStates, DDS_InstanceStateMask instanceStates);
 *
 * @ingroup CSubscription
 *
 * @brief   ���ҵײ㺬�д����ض�״̬���������������ݶ��ߵĽ�ϡ�
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in,out]  readers  ���ڲ��������ڴ洢�������������ݶ����б�
 * @param   sampleStates       ��Ҫ���������״̬������
 * @param   viewStates         ��Ҫ�������ͼ״̬������
 * @param   instanceStates     ��Ҫ�����ʵ��״̬������
 *
 * @details  �������õ��������� DDS_SampleStateKind ��DDS_ViewStateKind �� DDS_InstanceStateKind
 *           ��ͨ����������ʾ״̬�ļ��ϡ����ݶ�����ֻҪ��һ����������������ݶ��߷���������
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *          - #DDS_RETCODE_OK ����ȡ�ɹ���
 *          - #DDS_RETCODE_OUT_OF_RESOURCES ����@e readers �ṩ�Ŀռ䲻�㣬�ҵײ�����ʧ�ܣ�
 *          - #DDS_RETCODE_NOT_ENABLED ������������δʹ�ܣ�
 *          - #DDS_RETCODE_ERROR ���ڲ�������ϸ�μ���־��
 */

DCPSDLL DDS_ReturnCode_t DDS_Subscriber_get_datareaders(
    DDS_Subscriber* self,
    DDS_DataReaderSeq* readers,
    DDS_SampleStateMask sampleStates,
    DDS_ViewStateMask viewStates,
    DDS_InstanceStateMask instanceStates);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Subscriber_notify_datareaders( DDS_Subscriber* self);
 *
 * @ingroup CSubscription
 *
 * @brief   �����������Ե������д��� #DDS_DATA_AVAILABLE_STATUS ״̬�����ݶ��߹����ļ�������
 *          DDS_DataReaderListener::on_data_available ������
 *
 * @details �����ݶ��ߵײ����µ����ݵ���ʱ�����ᴦ�� #DDS_DATA_AVAILABLE_STATUS ״̬�����û��ص��ɹ������û�
 *          ͨ�� @ref read-take ϵ�з�����ȡ����ʱ�������״̬���÷���ͨ����
 *          DDS_SusbcriberListener::data_on_reader �ص�������ʹ�á�
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * 
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK ��֪ͨ�ɹ���
 *          - #DDS_RETCODE_NOT_ENABLED ������������δʹ�ܣ�
 *          - #DDS_RETCODE_ERROR ���ڲ�������ϸ�μ���־��
 */

DCPSDLL DDS_ReturnCode_t DDS_Subscriber_notify_datareaders(
    DDS_Subscriber* self);

/**
 * @fn  DCPSDLL DDS_DomainParticipant* DDS_Subscriber_get_participant( DDS_Subscriber* self);
 *
 * @ingroup CSubscription
 *
 * @brief   ��ö����ߵĸ�ʵ��������ߡ�
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * 
 * @return  ���ظö����ߵĸ�ʵ���������ߡ�
 */

DCPSDLL DDS_DomainParticipant* DDS_Subscriber_get_participant(
    DDS_Subscriber* self);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Subscriber_set_default_datareader_qos( DDS_Subscriber* self, const DDS_DataReaderQos* qoslist);
 *
 * @ingroup CSubscription
 *
 * @brief   �÷�������Ϊ���ݶ��߱����Ĭ��QoS���á�
 *
 * @details Ĭ�ϵ�QoS�ڴ����µ����ݶ���ʱָ��QoS����Ϊ #DDS_DATAREADER_QOS_DEFAULT ʱʹ�õ�QoS���ã�
 *          ʹ�������ֵ #DDS_DATAREADER_QOS_DEFAULT ������QoS�еĸ������õ�����ΪĬ��ֵ��
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   qoslist ָ��QoS���á�
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK :��ʾ���óɹ���
 *          - #DDS_RETCODE_BAD_PARAMETER :��ʾ@e qoslistΪ�գ�����@e qoslist ������Чֵ��
 *          - #DDS_RETCODE_INCONSISTENT :��ʾ@e qoslist �о��в����ݵ����ã�
 *          - #DDS_RETCODE_ERROR :��ʾʧ�ܣ����縴��QoSʱ��������
 */

DCPSDLL DDS_ReturnCode_t DDS_Subscriber_set_default_datareader_qos(
    DDS_Subscriber* self, 
    const DDS_DataReaderQos* qoslist);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Subscriber_get_default_datareader_qos( DDS_Subscriber* self, DDS_DataReaderQos* qoslist);
 *
 * @ingroup CSubscription
 *
 * @brief   �÷�����ȡΪ���ݶ��߱����Ĭ��QoS���á�
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in,out]  qoslist ���ڲ�����ʾ��ȡ�Ľ��.
 *
 * @return  ��ǰ�ķ���ֵ���ͣ�
 *          - #DDS_RETCODE_OK :��ʾ���óɹ���
 *          - #DDS_RETCODE_ERROR :��ʾʧ�ܣ����縴��QoSʱ��������
 */

DCPSDLL DDS_ReturnCode_t DDS_Subscriber_get_default_datareader_qos(
    DDS_Subscriber* self, 
    DDS_DataReaderQos* qoslist);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Subscriber_copy_from_topic_qos( DDS_DataReaderQos* datareaderQos, const DDS_TopicQos* topicQos);
 *
 * @ingroup CSubscription
 *
 * @brief   ������QoS�й�����Ӧ�����ݶ���QoS��
 *
 * @param [in,out]  datareaderQos    ���ڲ�������ʾ����Ľ�����ݶ�ȡQoS���á�
 * @param   topicQos                 Դ����QoS���á�
 *
 * @return  ���ر�ʾ��������ķ����룺
 *          - #DDS_RETCODE_OK ������ɹ���
 *          - #DDS_RETCODE_ERROR ���ڲ����󣬾���μ���־�ļ������ܵ�ԭ��Ϊ�����ڴ�ʧ�ܡ�
 */

DCPSDLL DDS_ReturnCode_t DDS_Subscriber_copy_from_topic_qos(
    DDS_DataReaderQos* datareaderQos, 
    const DDS_TopicQos* topicQos);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Subscriber_delete_contained_entities( DDS_Subscriber* self);
 *
 * @ingroup CSubscription
 *
 * @brief   �÷���ɾ���ö����ߴ���������ʵ�塣
 *
 * @details �÷��������ʵ����еݹ���� delete_contained_entities ���������յ�ɾ����ʵ��������ݶ��ߡ���ȡ�����ȣ�
 *          �÷�����ȡ������Ϊ�Ĳ���ɾ����������ɾ��������ʵ�����ɾ��������в�����ɾ��������ʵ�壬�򷵻��ض��Ĵ����롣
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 *                  
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK :��ʾɾ���ɹ���
 *          - #DDS_RETCODE_PRECONDITION_NOT_MET :��ʾ�в���ʵ�岻����ɾ����������ɾ������ʵ�壻
 *          - #DDS_RETCODE_ERROR :��ʾδ���������ϸ�μ���־��
 */

DCPSDLL DDS_ReturnCode_t DDS_Subscriber_delete_contained_entities(
    DDS_Subscriber* self);

/**
 * @fn  DCPSDLL DDS_Entity* DDS_Subscriber_as_entity(DDS_Subscriber* self);
 *
 * @ingroup CSubscription
 *
 * @brief   ��������ת��Ϊ�����ࡱʵ�����
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 *
 * @return  �ձ�ʾת��ʧ�ܣ�����ָ�򡰸��ࡱʵ�����
 */

DCPSDLL DDS_Entity* DDS_Subscriber_as_entity(DDS_Subscriber* self);

/**
 * @struct DDS_SubscriberSeq 
 *
 * @ingroup CSubscription
 *
 * @brief   ���� DDS_Subscriber ָ����������ͣ��μ� #DDS_USER_SEQUENCE_C ��
 */
DDS_SEQUENCE_C(DDS_SubscriberSeq, DDS_Subscriber*);

#ifdef _ZRXMLINTERFACE

#ifdef _ZRXMLENTITYINTERFACE

DCPSDLL DDS_ReturnCode_t DDS_Subscriber_lookup_named_datareaders(
    DDS_Subscriber* self,
    const char* pattern, DDS_StringSeq* reader_names);

DCPSDLL DDS_DataReader* DDS_Subscriber_lookup_datareader_by_name(
    DDS_Subscriber* self, const DDS_Char* name);

DCPSDLL const DDS_Char* DDS_Subscriber_get_entity_name(
    DDS_Subscriber* self);

DCPSDLL DDS_DomainParticipant* DDS_Subscriber_get_factory(
    DDS_Subscriber* self);

DCPSDLL DDS_DataReader* DDS_Subscriber_create_datareader_from_xml_string(
    DDS_Subscriber* self,
    const DDS_Char* xml_content);

DCPSDLL DDS_ReturnCode_t DDS_Subscriber_to_xml(
    DDS_Subscriber* self,
    const DDS_Char** result,
    DDS_Boolean contained_qos);

#endif /* _ZRXMLENTITYINTERFACE */

#ifdef _ZRXMLQOSINTERFACE

/**
 * @fn  DCPSDLL DDS_DataReader* DDS_Subscriber_create_datareader_with_topic_and_qos_profile( DDS_Subscriber* self, const DDS_Char* topicName, DDS_TypeSupport* typeSupport, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name, DDS_DataReaderListener* drListener, DDS_StatusKindMask mask) DCPSDLL DDS_DataReader* DDS_Subscriber_create_datareader_with_qos_profile( DDS_Subscriber* self, DDS_TopicDescription *topic, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name, DDS_DataReaderListener *dr_listener, DDS_StatusKindMask mask);
 *
 * @ingroup CSubscription
 *
 * @brief   ����ָ���������Ƶ����ݶ��ߣ����������ƹ���������δ����ʱ�����Զ����������������Ѿ����������ⴴ�����ݶ��ߡ�
 *
 * @param [in,out]  self        ָ�������ߡ�
 * @param   topicName           ���ݶ��߹������������ơ�
 * @param [in,out]  typeSupport ���ݶ��߹������������͵�����֧��ȫ�ֶ����ַ��DDS��Ϊÿ���������;�����һ��ȫ�ֶ��󣬶������ƹ���Ϊ�� ��������TypeSupport_instance �����㿽�����ͣ� DDS_ZeroCopyBytesTypeSupport_instance ��
 * @param   library_name        QoS������֣�������ΪNULL��
 * @param   profile_name        QoS���õ����֣�������ΪNULL��
 * @param   qos_name            QoS�����֣�����ΪNULL����ת��Ϊdefault�ַ�����
 * @param [in,out]  drListener  ���ݶ��ߵļ�������
 * @param   mask                ���������롣
 *
 * @return  NULL��ʾʧ�ܣ����򷵻����ݶ���ָ�롣
 */

DCPSDLL DDS_DataReader* DDS_Subscriber_create_datareader_with_topic_and_qos_profile(
    DDS_Subscriber* self,
    const DDS_Char* topicName,
    DDS_TypeSupport* typeSupport,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name,
    DDS_DataReaderListener* drListener,
    DDS_StatusKindMask mask);

/**
 * @fn  DCPSDLL DDS_DataReader* DDS_Subscriber_create_datareader_with_qos_profile( DDS_Subscriber* self, DDS_TopicDescription *topic, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name, DDS_DataReaderListener *dr_listener, DDS_StatusKindMask mask);
 *
 * @ingroup CSubscription
 *
 * @brief   ��QoS�ֿ��л�ȡ���ݶ���QoS�����䴴�����ݶ���
 *
 * @param [in,out]  self        ָ��Ŀ��
 * @param [in,out]  topic       ���ݶ��߹���������
 * @param   library_name        QoS������֣�������ΪNULL��
 * @param   profile_name        QoS���õ����֣�������ΪNULL��
 * @param   qos_name            QoS�����֣�����ΪNULL����ת��Ϊdefault�ַ�����
 * @param [in,out]  dr_listener Ϊ�ö��������õļ��������˲�������Ϊ�ա� ZRDDS����ӹܼ�����������ڴ�������û������ͷš�
 * @param   mask                ����Ӧ�ó������Ȥ��״̬��ֻ��Ӧ�ó������Ȥ��״̬�����仯ʱ���Ż�֪ͨӦ�ó���
 *
 * @return  ��NULL��ʾ�����ɹ��������ʾʧ�ܣ�ʧ�ܵ�ԭ�����Ϊ��
 *          - @e a_topic ������Ч���������
 *          - @e a_topic �ĸ�ʵ����ö����߲�����һ���������ʵ�壻
 *          - @e library_name @e profile_name @e qos_name ָ����QoS�к�����Ч��QoS���ߺ��в�һ�µ�QoS���ã�
 *          - �����ڴ�����δ���������ϸ�μ���־��
 */
DCPSDLL DDS_DataReader* DDS_Subscriber_create_datareader_with_qos_profile(
    DDS_Subscriber* self,
    DDS_TopicDescription *topic,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name,
    DDS_DataReaderListener *dr_listener,
    DDS_StatusKindMask mask);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Subscriber_set_default_datareader_qos_with_profile( DDS_Subscriber* self, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name);
 *
 * @ingroup CSubscription
 *
 * @brief   ��QoS�ֿ��л�ȡ���ݶ���QoS����������ΪĬ��DataReaderQos
 *
 * @param [in,out]  self    ָ��Ŀ��
 * @param   library_name    QoS������֣�������ΪNULL��
 * @param   profile_name    QoS���õ����֣�������ΪNULL��
 * @param   qos_name        QoS�����֣�����ΪNULL����ת��Ϊdefault�ַ�����
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *         - #DDS_RETCODE_OK :��ʾ���óɹ���
 *         - #DDS_RETCODE_ERROR :��ʾδ֪�����µ����ô���
 */
DCPSDLL DDS_ReturnCode_t DDS_Subscriber_set_default_datareader_qos_with_profile(
    DDS_Subscriber* self,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Subscriber_set_qos_with_profile( DDS_Subscriber* self, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name);
 *
 * @ingroup CSubscription
 *
 * @brief   ��QoS�ֿ��л�ȡ������QoS���������õ���������
 *
 * @param [in,out]  self    ָ��Ŀ��
 * @param   library_name    QoS������֣�������ΪNULL��
 * @param   profile_name    QoS���õ����֣�������ΪNULL��
 * @param   qos_name        QoS�����֣�����ΪNULL����ת��Ϊdefault�ַ�����
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *         - #DDS_RETCODE_OK :��ʾ���óɹ���
 *         - #DDS_RETCODE_ERROR :��ʾδ֪�����µ����ô���
 */
DCPSDLL DDS_ReturnCode_t DDS_Subscriber_set_qos_with_profile(
    DDS_Subscriber* self,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name);

#endif /* _ZRXMLQOSINTERFACE */

#endif /* _ZRXMLINTERFACE */

#ifdef __cplusplus
}
#endif

#endif /* Subscriber_h__*/
