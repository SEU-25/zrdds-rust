/**
 * @file:       DomainParticipant.h
 *
 * copyright:   Copyright (c) 2018 ZhenRong Technology, Inc. All rights reserved.
 */

#ifndef DDS_DomainParticipant_h__
#define DDS_DomainParticipant_h__

#include "OsResource.h"
#include "DomainId_t.h"
#include "ReturnCode_t.h"
#include "DomainParticipantQos.h"
#include "PublisherQos.h"
#include "SubscriberQos.h"
#include "TopicQos.h"
#include "DomainParticipantListener.h"
#include "PublisherListener.h"
#include "SubscriberListener.h"
#include "TopicListener.h"
#include "StatusKindMask.h"
#include "InstanceHandle_t.h"
#include "ParticipantBuiltinTopicData.h"
#include "Publisher.h"
#include "Subscriber.h"
#include "Topic.h"
#include "ContentFilteredTopic.h"
#include "TopicBuiltinTopicData.h"
#include "ZRDDSTypeSupport.h"

#ifdef __cplusplus
extern "C"
{
#endif

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_get_qos( DDS_DomainParticipant* self, DDS_DomainParticipantQos* qoslist);
 *
 * @ingroup CDomain
 *
 * @brief   ��ȡ��������ߵ�QoS���á�
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 * @param [in,out]  qoslist ���ڲ��������ڱ���������ߵ�QoS���á�
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK :��ʾ��ȡ�ɹ���
 *          - #DDS_RETCODE_ERROR :��ʾʧ�ܣ�ԭ�����Ϊ����QoSʱ��������
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_get_qos(
    DDS_DomainParticipant* self, 
    DDS_DomainParticipantQos* qoslist);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_set_qos( DDS_DomainParticipant* self, const DDS_DomainParticipantQos* qoslist);
 *
 * @ingroup CDomain
 *
 * @brief   �÷�������Ϊ����������õ�QoS��
 *
 * @details ����ʹ������ֵ #DDS_DOMAINPARTICIPANT_QOS_DEFAULT ��ʾʹ�ô洢��������߹����е�QoS���á�
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 * @param   qoslist ��ʾ�û���Ҫ���õ�QoS���á�
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK :��ʾ��ȡ�ɹ���
 *          - #DDS_RETCODE_BAD_PARAMETER :��ʾ @e qoslist ������Ч��QoS���ã�
 *          - #DDS_RETCODE_INCONSISTENT :��ʾ @e qoslist ���в����ݵ�QoS���ã�
 *          - #DDS_RETCODE_IMMUTABLE_POLICY :��ʾ�û������޸�ʹ�ܺ󲻿ɱ��QoS���ã�
 *          - #DDS_RETCODE_ERROR :��ʾδ����Ĵ��󣬴�����ϸ��Ϣ�������־�У�
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_set_qos(
    DDS_DomainParticipant* self, 
    const DDS_DomainParticipantQos* qoslist);

/**
 * @fn  DCPSDLL DDS_DomainParticipantListener* DDS_DomainParticipant_get_listener( DDS_DomainParticipant* self);
 *
 * @ingroup CDomain
 *
 * @brief   �÷�����ȡͨ�� #DDS_DomainParticipant_set_listener �������ߴ���ʱΪ����������õļ���������
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - NULL��ʾδ���ü�������
 *          - �ǿձ�ʾӦ��ͨ�� #DDS_DomainParticipant_set_listener �����ڴ���ʱ���õļ���������
 */

DCPSDLL DDS_DomainParticipantListener* DDS_DomainParticipant_get_listener(
    DDS_DomainParticipant* self);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_set_listener( DDS_DomainParticipant* self, DDS_DomainParticipantListener* listener, const DDS_StatusKindMask mask);
 *
 * @ingroup CDomain
 *
 * @brief   ���ø�������ߵļ�������
 *
 * @details  ������������ԭ�м�������������ÿն����ʾ���ԭ�����õļ�������
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 * @param [in,out]  listener    Ϊ������������õļ���������
 * @param   mask                ����Ӧ�ó������Ȥ��״̬��ֻ��Ӧ�ó������Ȥ��״̬�����仯ʱ���Ż�֪ͨӦ�ó���
 *
 * @return  ��ǰ���Ƿ��� #DDS_RETCODE_OK ��ʾ���óɹ���
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_set_listener(
    DDS_DomainParticipant* self, 
    DDS_DomainParticipantListener* listener, 
    const DDS_StatusKindMask mask);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_enable( DDS_DomainParticipant* self);
 *
 * @ingroup CDomain
 *
 * @brief   �ֶ�ʹ�ܸ�ʵ�壬�μ�@ref entity-enable ��
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK ����ʾ��ȡ�ɹ���
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_enable(
    DDS_DomainParticipant* self);

/**
 * @fn  DCPSDLL DDS_Publisher* DDS_DomainParticipant_create_publisher( DDS_DomainParticipant* self, const DDS_PublisherQos* qoslist, DDS_PublisherListener* listener, DDS_StatusKindMask mask);
 *
 * @ingroup CDomain
 *
 * @brief   �÷�������������´���һ����������ʵ�壬������QoS�Լ�������������Ӧ����Ҫ������ڷ������ݡ�
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 * @param   qoslist             ��ʾΪ�÷��������õ�QoS�� #DDS_PUBLISHER_QOS_DEFAULT ����ʹ�ø���������б����Ĭ�ϵ�QoS��
 * @param [in,out]  listener    Ϊ�÷��������õļ�������
 * @param   mask                ����Ӧ�ó������Ȥ��״̬��ֻ��Ӧ�ó������Ȥ��״̬�����仯ʱ���Ż�֪ͨӦ�ó���
 *
 * @return  �����ɹ�ָ�򴴽��ɹ��ķ�����ʵ����󣬷��򷵻�NULL��ʧ�ܵ�ԭ�����Ϊ��
 *          - ����ռ�ʧ�ܻ��߳�ʼ����Դʧ�ܣ�����Ĵ�����Ϣ�μ���־��
 *          - @e qoslist ������Чֵ���ߺ��в�һ�µ�QoS��
 */

DCPSDLL DDS_Publisher* DDS_DomainParticipant_create_publisher(
    DDS_DomainParticipant* self,
    const DDS_PublisherQos* qoslist,
    DDS_PublisherListener* listener,
    DDS_StatusKindMask mask);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_get_publishers( DDS_DomainParticipant* self, DDS_PublisherSeq* publishers);
 *
 * @ingroup CDomain
 *
 * @brief   ��ȡ��ǰδ��ɾ�����ɸ�������ߴ����ķ������б�
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 * @param [in,out]  publishers  ���ڲ�����������䷢����ʵ�����
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *          - #DDS_RETCODE_OK :���ڲ�����Ч����ʾ��ȡ�ɹ���
 *          - #DDS_RETCODE_OUT_OF_RESOURCES ����ʾ�û��ṩ�Ŀռ䲻��������ʧ�ܣ�
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_get_publishers(
    DDS_DomainParticipant* self,
    DDS_PublisherSeq* publishers);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_delete_publisher( DDS_DomainParticipant* self, DDS_Publisher* publisher);
 *
 * @ingroup CDomain
 *
 * @brief   ɾ��ָ���ķ�����ʵ�塣
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 * @param [in,out]  publisher   ָ���ķ����ߡ�
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_BAD_PARAMETER :��ʾ����Ĳ�����Ч������Ч�ķ�����ָ�룻
 *              - ��Ч�ķ�����ָ�룬���ǲ����ڸ�������ߴ��������ġ�
 *          - #DDS_RETCODE_PRECONDITION_NOT_MET :
 *              - ��ʾ�÷����߲�����ɾ����������������ʵ������д��δɾ����
 *              - ����ķ����߲��ɸ�������ߴ�����
 *          - #DDS_RETCODE_OK ����ʾɾ���ɹ���
 *          - #DDS_RETCODE_ERROR ��δ����Ĵ�����ϸ��Ϣ�μ���־��
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_delete_publisher(
    DDS_DomainParticipant* self,
    DDS_Publisher* publisher);

/**
 * @fn  DCPSDLL DDS_Subscriber* DDS_DomainParticipant_create_subscriber( DDS_DomainParticipant* self, const DDS_SubscriberQos* qoslist, DDS_SubscriberListener* listener, DDS_StatusKindMask mask);
 *
 * @ingroup CDomain
 *
 * @brief   �÷�������������´���һ����������ʵ�壬������QoS�Լ�������������Ӧ����Ҫ������ڶ������ݡ�
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 * @param   qoslist             ��ʾΪ�ö��������õ�QoS�� #DDS_SUBSCRIBER_QOS_DEFAULT ����ʹ�ø���������б����Ĭ�ϵ�QoS��
 * @param [in,out]  listener    Ϊ�ö��������õļ�������
 * @param   mask                ����Ӧ�ó������Ȥ��״̬��ֻ��Ӧ�ó������Ȥ��״̬�����仯ʱ���Ż�֪ͨӦ�ó���
 *
 * @return  �����ɹ�ָ�򴴽��ɹ��Ķ�����ʵ����󣬷��򷵻�NULL��ʧ�ܵ�ԭ�����Ϊ��
 *          - ����ռ�ʧ�ܻ��߳�ʼ����Դʧ�ܣ�����Ĵ�����Ϣ�μ���־��
 *          - @e qoslist ������Чֵ���ߺ��в�һ�µ�QoS��
 */

DCPSDLL DDS_Subscriber* DDS_DomainParticipant_create_subscriber(
    DDS_DomainParticipant* self,
    const DDS_SubscriberQos* qoslist,
    DDS_SubscriberListener* listener,
    DDS_StatusKindMask mask);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_get_subscribers( DDS_DomainParticipant* self, DDS_SubscriberSeq* subscribers);
 *
 * @ingroup CDomain
 *
 * @brief   ��ȡ��ǰδ��ɾ�����ɸ�������ߴ����Ķ������б�
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 * @param [in,out]  subscribers ���ڲ�����������䶩����ʵ�����
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *          - #DDS_RETCODE_OK :���ڲ�����Ч����ʾ��ȡ�ɹ���
 *          - #DDS_RETCODE_OUT_OF_RESOURCES ����ʾ�û��ṩ�Ŀռ䲻��������ʧ�ܣ�
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_get_subscribers(
    DDS_DomainParticipant* self,
    DDS_SubscriberSeq* subscribers);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_delete_subscriber( DDS_DomainParticipant* self, DDS_Subscriber* subscriber);
 *
 * @ingroup CDomain
 *
 * @brief   ɾ��ָ���Ķ�����ʵ�塣
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 * @param [in,out]  subscriber  ָ���Ķ����ߡ�
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_BAD_PARAMETER :��ʾ����Ĳ�����Ч������Ч�ķ�����ָ�룻
 *              - ��Ч�Ķ�����ָ�룬���ǲ����ڸ�������ߴ��������ġ�
 *          - #DDS_RETCODE_PRECONDITION_NOT_MET :
 *              - ��ʾ�ö����߲�����ɾ����������������ʵ�����ݶ���δɾ����
 *              - ����Ķ����߲��ɸ�������ߴ�����
 *          - #DDS_RETCODE_OK ����ʾɾ���ɹ���
 *          - #DDS_RETCODE_ERROR ��δ����Ĵ�����ϸ��Ϣ�μ���־��
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_delete_subscriber(
    DDS_DomainParticipant* self,
    DDS_Subscriber* subscriber);

/**
 * @fn  DCPSDLL DDS_Topic* DDS_DomainParticipant_create_topic( DDS_DomainParticipant* self, const DDS_Char* topicName, const DDS_Char* typeName, const DDS_TopicQos* qoslist, DDS_TopicListener* listener, DDS_StatusKindMask mask);
 *
 * @ingroup CDomain
 *
 * @brief   �÷�������������´���һ��������ʵ�壬����������Ψһ�����ơ��������������͡�QoS�Լ������������ڳ������ڵ����ݡ�
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 * @param   topicName           �´�������������ƣ���֤�������������Ψһ��
 * @param   typeName            �´�����������������͵����ƣ����������Ʊ���������������ע����������ƣ�
 *                              ע������ע��������Լ����ͱ�������ƣ���ע��ķ���Ϊʹ�ñ��������ɵ�֧�ֽӿ�
 *                               #FooTypeSupport_register_type ��
 * @param   qoslist             �´����������QoS���á�
 * @param [in,out]  listener    ������Ϣ�ص��ӿڡ�
 * @param   mask                �ص���Ϣ�������롣
 *
 * @return  �����ɹ���ָ�򴴽��ɹ���������󣬷��򷵻�NULL��ԭ��������£�
 *          - ����ռ�ʧ�ܣ�
 *          - ����Ĳ����Ƿ��� @e topicName == NULL @e typeName == NULL����
 *          - �����QoS������Чֵ����QoS�к��в�һ�µ����ã�
 *          - ����Ĳ���δע�᣻
 *          - �����������������ͬ�����������⣻
 *          - δ���������ϸ�μ���־��
 */

DCPSDLL DDS_Topic* DDS_DomainParticipant_create_topic(
    DDS_DomainParticipant* self,
    const DDS_Char* topicName,
    const DDS_Char* typeName,
    const DDS_TopicQos* qoslist,
    DDS_TopicListener* listener,
    DDS_StatusKindMask mask);

/**
 * @fn  DCPSDLL DDS_Topic* DDS_DomainParticipant_create_topic_w_type_support( DDS_DomainParticipant* self, const DDS_Char* topicName, DDS_TypeSupport* typesupport, DDS_TopicQos* qoslist, DDS_TopicListener* listener, const DDS_StatusKindMask mask);
 *
 * @ingroup  CDomain
 *
 * @brief   �������⣬���ڴ���֮ǰ�Զ�ע�����⡣
 *
 * @param [in,out]  self        �������
 * @param   topicName           �������ơ�
 * @param [in,out]  typesupport ����������������͵�����֧��ȫ�ֶ����ַ��DDS��Ϊÿ���������;�����һ��ȫ�ֶ��󣬶������ƹ���Ϊ�� ��������TypeSupport_instance �����㿽�����ͣ� DDS_ZeroCopyBytesTypeSupport_instance ��
 * @param [in,out]  qoslist     Qos���ԡ�
 * @param [in,out]  listener    ��������
 * @param   mask                ���������롣
 *
 * @return  NULL��ʾʧ�ܣ����򷵻�����ָ�롣
 *
 */

DCPSDLL DDS_Topic* DDS_DomainParticipant_create_topic_w_type_support(
    DDS_DomainParticipant* self,
    const DDS_Char* topicName,
    DDS_TypeSupport* typesupport,
    DDS_TopicQos* qoslist,
    DDS_TopicListener* listener,
    const DDS_StatusKindMask mask);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_delete_topic( DDS_DomainParticipant* self, DDS_Topic* topic);
 *
 * @ingroup CDomain
 *
 * @brief   ɾ��ָ�����⡣
 *
 * @details �ڵ��ø÷���֮ǰ��Ҫ��֤�����������������ʵ�壨����д�ߡ����ݶ��ߡ������ڹ��˵����⣩���Ѿ���ɾ����
 *          �������������ô���Ϊ0�Σ��û�ͨ�� #DDS_DomainParticipant_find_topic ������������������ô�����
 *          ��������������������᷵�ش��� #DDS_RETCODE_PRECONDITION_NOT_MET ��
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in,out]  topic   ָ�����������
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *          - #DDS_RETCODE_OK :��ʾɾ���ɹ���
 *          - #DDS_RETCODE_BAD_PARAMETER :@e topic ������Ч���������
 *          - #DDS_RETCODE_PRECONDITION_NOT_MET
 *              - ��ʾָ�����ⲻ���ɸ�������ߴ����ģ�
 *              - ������ɾ��������
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_delete_topic(
    DDS_DomainParticipant* self, 
    DDS_Topic* topic);

#ifdef _ZRDDS_INCLUDE_CONTENTFILTER_TOPIC

/**
 * @fn  DCPSDLL DDS_ContentFilteredTopic* DDS_DomainParticipant_create_contentfilteredtopic( DDS_DomainParticipant* self, const DDS_Char* name, DDS_Topic* relatedTopic, const DDS_Char* filterExp, const DDS_StringSeq* filterPara);
 *
 * @ingroup CDomain
 *
 * @brief   �����������ݹ��˵����⡣
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   name            �������ݹ��˵��������ƣ����������Ʋ���ͨ���������ݴ��ݵ�Զ�̣����ڱ���ʹ�ã�name
 *                          Ӧ�ñ�֤�����������Ψһ��������ͨ��������ƣ���
 * @param   relatedTopic   �����Ļ������⡣
 * @param   filterExp      ���˱��ʽ���﷨����μ� @ref expression-grammer ��
 * @param   filterPara     ���˲���������˱��ʽ���ʹ�á�
 *
 * @return  ��NULL��ʾ�����ɹ���NULL��ʾ����ʧ�ܣ�ʧ�ܵ�ԭ�����Ϊ��
 *          - �����Ѵ���ͬ�������⣻
 *          - �����Ļ������ⲻ���ڣ�
 *          - ���˱��ʽ���߹��˲������Ϸ���
 *          - �����ڴ�ʧ�ܡ�
 */

DCPSDLL DDS_ContentFilteredTopic* DDS_DomainParticipant_create_contentfilteredtopic(
    DDS_DomainParticipant* self,
    const DDS_Char* name,
    DDS_Topic* relatedTopic,
    const DDS_Char* filterExp,
    const DDS_StringSeq* filterPara);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_delete_contentfilteredtopic( DDS_DomainParticipant* self, DDS_ContentFilteredTopic* topic);
 *
 * @ingroup CDomain
 *
 * @brief   ɾ��ָ���������ݹ��˵����⡣
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in,out]  topic   ָ��Ŀ�ꡣ
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK ��ɾ���ɹ���
 *          - #DDS_RETCODE_BAD_PARAMETER ��������Ч���߲����ɸ�������ߴ�����
 *          - #DDS_RETCODE_PRECONDITION_NOT_MET ��������ɾ�����������������������������ݶ�����δɾ����
 *          - #DDS_RETCODE_ERROR ��ɾ��������ϸ�μ���־��
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_delete_contentfilteredtopic(
    DDS_DomainParticipant* self,
    DDS_ContentFilteredTopic* topic);

#endif /* _ZRDDS_INCLUDE_CONTENTFILTER_TOPIC */

/**
 * @fn  DCPSDLL DDS_Topic* DDS_DomainParticipant_find_topic( DDS_DomainParticipant* self, const DDS_Char* topicName, const DDS_Duration_t* timeout);
 *
 * @ingroup CDomain
 *
 * @brief   �������������������ұ������⡣
 *
 * @details �����������������Ѿ����ڣ���ֱ�ӷ��أ�����ȴ���ֱ����ʱ�����������������ⱻ������ע����ҳɹ�ʱ��
 *          �����Ӳ��ҳɹ����������������Ӧ���� #DDS_DomainParticipant_delete_topic ɾ����������
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   topicName   �������ơ�
 * @param   timeout     ��ȴ�ʱ�䡣
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - NULL��ʾ����ʧ�ܣ�ָ����ʱ����û���������������⡣
 *          - ��NULL��ʾ���ҵ�������������ָ�����
 */

DCPSDLL DDS_Topic* DDS_DomainParticipant_find_topic(
    DDS_DomainParticipant* self, 
    const DDS_Char* topicName,
    const DDS_Duration_t* timeout);

/**
 * @fn  DCPSDLL DDS_TopicDescription* DDS_DomainParticipant_lookup_topicdescription( DDS_DomainParticipant* self, const DDS_Char* topicName);
 *
 * @ingroup CDomain
 *
 * @brief   �����������ֲ��ұ��ش��������⣬���������Լ��������ݹ��˵����⡣
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   topicName       ָ����Ҫ���ҵ��������ơ�
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - NULL��ʾ����ʧ�ܣ�������δ����ָ�������������⡣
 *          - ��NULL��ʾ���ҵ������������ĸ���ָ�롣
 */

DCPSDLL DDS_TopicDescription* DDS_DomainParticipant_lookup_topicdescription(
    DDS_DomainParticipant* self, 
    const DDS_Char* topicName);

/**
 * @fn  DCPSDLL DDS_Subscriber* DDS_DomainParticipant_get_builtin_subscriber( DDS_DomainParticipant* self);
 *
 * @ingroup CDomain
 *
 * @brief   ��ȡ�û�ʵ�巢�ֵ����ö����ߡ�
 *
 * @details ����ʵ����ָ����DDS�м���ڲ����ݽ�����������Ϣ���������Ϣ�������⡢����д���Լ����ݶ��ߡ�
 *          Ϊ���ܹ���ȡ��������Ϣ������������ṩ�����ڲ������ߵĽӿڣ��û���ͨ�������ö����ߵĹ��ܻ�ȡ��Ҫ�ķ�����Ϣ��
 *          ����ʵ������������Զ���������ͨ���ýӿ���� DDS_Subscriber_lookup_datareader �ӿڻ�ȡ���õ�
 *          ���ݶ��ߣ�ʹ�����õ����ݶ��߳����������������û��Զ������ݶ��߲�֮ͬ�⣬�����������û��Զ������ݶ���һ�¡�
 *          �ṩ���û�ʹ�õ��������ݶ��߼���������������Ƽ���������������Ͳμ��±�
 *          �������ݶ��� | �������� | ��������
 *          --- | --- | ---
 *          DDS_ParticipantBuiltinTopicDataDataReader | #BUILTIN_PARTICIPANT_TOPIC_NAME | DDS_ParticipantBuiltinTopicData
 *          DDS_PublicationBuiltinTopicDataDataReader | #BUILTIN_PUBLICATION_TOPIC_NAME | DDS_PublicationBuiltinTopicData
 *          DDS_SubscriptionBuiltinTopicDataDataReader | #BUILTIN_SUBSCRIPTION_TOPIC_NAME | DDS_SubscriptionBuiltinTopicData
 *          ��ȡ��������ʵ�岻Ӧ��ɾ�����������ϵͳ�쳣������ʹ�õ����Ӳμ� @ref SetBuiltinListenerExample.c ��
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 *
 * @return  ���ö����ߵ�ָ�룬ZRDDS��ʵ���У�ֻҪ�������δ��ɾ������ô����ֵһ����Ч��
 */

DCPSDLL DDS_Subscriber* DDS_DomainParticipant_get_builtin_subscriber(
    DDS_DomainParticipant* self);

#ifdef _ZRDDS_INCLUDE_AUTO_CREATED_PUB_SUB

/**
 * @fn  DCPSDLL DDS_Publisher* DDS_DomainParticipant_get_auto_created_publisher( DDS_DomainParticipant* self);
 *
 * @ingroup CDomain
 *
 * @brief   Ϊ���û�������������ߴ���ʱ��ʹ��Ĭ��QoS�Զ������û�ʹ�õķ����ߣ��ýӿ�Ϊ��ȡ���Զ������ķ����ߡ�
 *
 * @param [in,out]  self    ָ��������ߡ�
 *
 * @return  �ɹ������Զ������ķ�����ָ�룬ʧ�ܷ���NULL��
 */

DCPSDLL DDS_Publisher* DDS_DomainParticipant_get_auto_created_publisher(
    DDS_DomainParticipant* self);

/**
 * @fn  DCPSDLL DDS_Subscriber* DDS_DomainParticipant_get_auto_created_subscriber( DDS_DomainParticipant* self);
 *
 * @ingroup CDomain
 *
 *
 * @brief   Ϊ���û�������������ߴ���ʱ��ʹ��Ĭ��QoS�Զ������û�ʹ�õĶ����ߣ��ýӿ�Ϊ��ȡ���Զ������Ķ����ߣ�
 *          �Զ������Ķ����������ö����߲�ͬ�����ö��������ڹ������õ�Readerʵ�壬���ڷ��ֹ��̡�
 *
 * @param [in,out]  self    ָ��������ߡ�
 *
 * @return  �ɹ������Զ������Ķ�����ָ�룬ʧ�ܷ���NULL��
 */

DCPSDLL DDS_Subscriber* DDS_DomainParticipant_get_auto_created_subscriber(
    DDS_DomainParticipant* self);

#ifdef _ZRXMLQOSINTERFACE

/**
 * @fn  DCPSDLL DDS_DataWriter* DDS_DomainParticipant_create_datawriter_with_topic_and_qos_profile( DDS_DomainParticipant* self, const DDS_Char* topicName, DDS_TypeSupport* typeSupport, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name, DDS_DataWriterListener* dwListener, DDS_StatusKindMask mask);
 *
 * @ingroup  CDomain
 *
 * @brief   �����Զ��������û������ߣ�����ָ���������Ƶ�����д�ߣ����������ƹ���������δ����ʱ�����Զ������� ���������Ѿ����������ⴴ������д�ߣ����øú����ȼ���
 *          DDS_DomainParticipant_get_auto_created_publisher()�Լ� DDS_Publisher_create_datawriter_with_topic_and_qos_profile ��
 *
 * @param [in,out]  self        ָ��������ߡ�
 * @param   topicName           ����д�߹������������ơ�
 * @param [in,out]  typeSupport ����д�߹������������͵�����֧��ȫ�ֶ����ַ��DDS��Ϊÿ���������;�����һ��ȫ�ֶ��󣬶������ƹ���Ϊ�� ��������TypeSupport_instance �����㿽�����ͣ� DDS_ZeroCopyBytesTypeSupport_instance ��
 * @param   library_name        QoS������֣�������ΪNULL��
 * @param   profile_name        QoS���õ����֣�������ΪNULL��
 * @param   qos_name            QoS�����֣�����ΪNULL����ת��Ϊdefault�ַ�����
 * @param [in,out]  dwListener  ����д�ߵļ�������
 * @param   mask                ���������롣
 *
 * @return  NULL��ʾʧ�ܣ����򷵻�����д��ָ�롣
 */

DCPSDLL DDS_DataWriter* DDS_DomainParticipant_create_datawriter_with_topic_and_qos_profile(
    DDS_DomainParticipant* self,
    const DDS_Char* topicName,
    DDS_TypeSupport* typeSupport,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name,
    DDS_DataWriterListener* dwListener,
    DDS_StatusKindMask mask);

/**
 * @fn  DCPSDLL DDS_DataReader* DDS_DomainParticipant_create_datareader_with_topic_and_qos_profile( DDS_DomainParticipant* self, const DDS_Char* topicName, DDS_TypeSupport* typeSupport, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name, DDS_DataReaderListener* drListener, DDS_StatusKindMask mask);
 *
 * @ingroup  CDomain
 *
 * @brief   �����Զ��������û������ߣ�����ָ���������Ƶ����ݶ��ߣ����������ƹ���������δ����ʱ�����Զ����������������Ѿ����������ⴴ�����ݶ��ߣ����øú����ȼ���
 *          DDS_DomainParticipant_get_auto_created_subscriber()�Լ�
 *          DDS_Subscriber_create_datareader_with_topic_and_qos_profile ��
 *
 * @param [in,out]  self        ָ��������ߡ�
 * @param   topicName           ���ݶ��߹������������ơ�
 * @param [in,out]  typeSupport ���ݶ��߹������������͵�����֧��ȫ�ֶ����ַ��DDS��Ϊÿ���������;�����һ��ȫ�ֶ��󣬶������ƹ���Ϊ�� ��������TypeSupport_instance �����㿽�����ͣ� DDS_ZeroCopyBytesTypeSupport_instance ��
 * @param   library_name        QoS������֣�������ΪNULL��
 * @param   profile_name        QoS���õ����֣�������ΪNULL��
 * @param   qos_name            QoS�����֣�����ΪNULL����ת��Ϊdefault�ַ�����
 * @param [in,out]  drListener  ���ݶ��ߵļ�������
 * @param   mask                ���������롣
 *
 * @return  NULL��ʾʧ�ܣ����򷵻�����д��ָ�롣.
 */

DCPSDLL DDS_DataReader* DDS_DomainParticipant_create_datareader_with_topic_and_qos_profile(
    DDS_DomainParticipant* self,
    const DDS_Char* topicName,
    DDS_TypeSupport* typeSupport,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name,
    DDS_DataReaderListener* drListener,
    DDS_StatusKindMask mask);

#endif /*_ZRXMLQOSINTERFACE*/

#endif /*_ZRDDS_INCLUDE_AUTO_CREATED_PUB_SUB*/

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_ignore_participant( DDS_DomainParticipant* self, const DDS_InstanceHandle_t* handle);
 *
 * @ingroup CDomain
 *
 * @brief   �÷������ں���ָ����������ߣ�����ͬδ���ָ�������ߣ����Ը���������µ����ж��ġ�������Ϣ��
 *
 * @details ͨ�Ź����ܵĳ���ʹ�ó���Ϊ���ʿ��ƣ���ͨ�����������ϸ��Ϣ�жϸ�������߻�������д�ߡ����ݶ����Ƿ�߱�
 *          ��Ӧ��Ȩ�ޣ����磺�������Я���������Ƿ�����Ҫ������Ѿ�������ָ����������ߣ��򽫽⿪���������ߵ�ƥ�䡣
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   handle  �û���ʶԶ�˵�������ߣ���Դ�μ� #DDS_DomainParticipant_get_discovered_participant_data ��
 *
 * @return  �Ƿ���Գɹ���
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_ignore_participant(
    DDS_DomainParticipant* self, 
    const DDS_InstanceHandle_t* handle);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_ignore_topic( DDS_DomainParticipant* self, const DDS_InstanceHandle_t* handle);
 *
 * @ingroup CDomain
 *
 * @brief   ����ָ����ʶ��ʾ��������������ж���/������
 *
 * @details ���������ݶ����Լ�����д�ߣ��������жԸ�����Ķ��Ļ��߷�����ƥ�䣬����Ѿ�ƥ������Ͽ������Ե�����д���Լ����ݶ���ƥ�䡣
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   handle  ��ʶָ�������⡣
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_NOT_ENABLED :��ʾδʹ�ܡ�
 *          - #DDS_RETCODE_ERROR :��ʾ���Դ���
 *          - #DDS_RETCODE_OK :��ʾ���Գɹ���
 *
 * @warning ZRDDS��ǰδʵ�ָýӿڡ�
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_ignore_topic(
    DDS_DomainParticipant* self, 
    const DDS_InstanceHandle_t* handle);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_ignore_publication( DDS_DomainParticipant* self, const DDS_InstanceHandle_t* handle);
 *
 * @ingroup CDomain
 *
 * @brief   ����ָ������д�ߣ������뱾�����ݶ���ƥ�䡣
 *
 * @details ���Է����Ƕ��ڱ��ص����ݶ�����˵�ģ��ڸ÷���������֮�󣬸���������µ��������ݶ��߾������յ����Ը�����д�ߵ����ݣ�
 *          ͨ���������ǣ����� #FooDataReader_get_matched_publications ����ȡ�Ѿ���Ե�Զ������д�ߵ�
 *          Ψһ��ʶ����ͨ�����ü�������ȡԶ������д�ߵ�Ψһ��ʶ���ٵ��øú��������ú��ԡ�
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   handle  ��ʶ��Ҫ�����Ե�����д�ߡ�
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK :��ʾ���Գɹ�������@e handle ָ��������д�߲����ڵ������
 *          - #DDS_RETCODE_BAD_PARAMETER :@e handle ������Ч������д�߱�ʶ��
 *          - #DDS_RETCODE_NOT_ENABLED :��ʾ���������δʹ�ܡ�
 *          - #DDS_RETCODE_ERROR :δ���������ϸ�μ���־��Ϣ��
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_ignore_publication(
    DDS_DomainParticipant* self, 
    const DDS_InstanceHandle_t* handle);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_ignore_subscription( DDS_DomainParticipant* self, const DDS_InstanceHandle_t* handle);
 *
 * @ingroup CDomain
 *
 * @brief   ����ָ�����ݶ��ߣ������뱾������д��ƥ�䡣
 *
 * @details ���Զ����Ƕ��ڱ��ص�����д����˵�ģ��ڸ÷���������֮�󣬸���������µ���������д�߾�����������ݶ��߷������ݣ�
 *          ͨ���������ǣ����� #FooDataWriter_get_matched_subscriptions ����ȡ�Ѿ���Ե�Զ�����ݶ��ߵ�
 *          Ψһ��ʶ����ͨ�����ü�������ȡԶ�����ݶ��ߵ�Ψһ��ʶ���ٵ��øú��������ú��ԡ�
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   handle  ��ʶ��Ҫ�����Ե����ݶ��ߡ�
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK :��ʾ���Գɹ�������@e handle ָ�������ݶ��߲����ڵ������
 *          - #DDS_RETCODE_BAD_PARAMETER :@e handle ������Ч�����ݶ��߱�ʶ��
 *          - #DDS_RETCODE_NOT_ENABLED :��ʾ���������δʹ�ܡ�
 *          - #DDS_RETCODE_ERROR :δ���������ϸ�μ���־��Ϣ��
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_ignore_subscription(
    DDS_DomainParticipant* self, 
    const DDS_InstanceHandle_t* handle);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_delete_contained_entities( DDS_DomainParticipant* self);
 *
 * @ingroup CDomain
 *
 * @brief   �÷���ɾ��������ߴ���������ʵ�塣
 *
 * @details ���������ߡ������ߡ����⡢�������ݹ��˵����⡣���Ҷ���ʵ����еݹ���� delete_contained_entities
 *          ���������յ�ɾ����ʵ���������д�ߡ����ݶ��ߡ���ȡ�����ȣ�ɾ����ʵ�岻��������ʵ�塣
 *          �÷�����ȡ������Ϊ�Ĳ���ɾ����������ɾ��������ʵ�����ɾ��������в�����ɾ��������ʵ�壬�򷵻��ض��Ĵ����롣
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK :��ʾɾ���ɹ���
 *          - #DDS_RETCODE_PRECONDITION_NOT_MET :��ʾ�в���ʵ�岻����ɾ����������ɾ������ʵ�壻
 *          - #DDS_RETCODE_ERROR :��ʾδ���������ϸ�μ���־��
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_delete_contained_entities(
    DDS_DomainParticipant* self);

#ifdef _ZRDDS_INCLUDE_LIVELINESS_QOS

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_assert_liveliness( DDS_DomainParticipant* self);
 *
 * @ingroup CDomain
 *
 * @brief   �÷��������ֶ�����������ߵĴ���ԣ�������Բ�������Ϊ #DDS_MANUAL_BY_PARTICIPANT_LIVELINESS_QOS ʱ���á�
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK :��ʾ��������Գɹ���
 *          - #DDS_RETCODE_NOT_ENABLED :��ʾ���������δʹ�ܡ�
 *          - #DDS_RETCODE_ERROR :�ڲ�δ���������ϸ�μ���־��
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_assert_liveliness(
    DDS_DomainParticipant* self);
#endif /* _ZRDDS_INCLUDE_LIVELINESS_QOS */

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_set_default_publisher_qos( DDS_DomainParticipant* self, const DDS_PublisherQos* qoslist);
 *
 * @ingroup CDomain
 *
 * @brief   �÷�������Ϊ�����߱����Ĭ��QoS���á�
 *
 * @details Ĭ�ϵ�QoS�ڴ����µĶ�����ʱָ��QoS����Ϊ #DDS_PUBLISHER_QOS_DEFAULT ʱʹ�õ�QoS���ã�
 *          ʹ�������ֵ #DDS_PUBLISHER_QOS_DEFAULT ������QoS�еĸ������õ�����ΪĬ��ֵ��
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

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_set_default_publisher_qos(
    DDS_DomainParticipant* self, 
    const DDS_PublisherQos* qoslist);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_get_default_publisher_qos( DDS_DomainParticipant* self, DDS_PublisherQos* qoslist);
 *
 * @ingroup CDomain
 *
 * @brief   �÷�����ȡΪ�����߱����Ĭ��QoS���á�
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in,out]  qoslist ���ڲ�����ʾ��ȡ�Ľ��.
 *
 * @return  ��ǰ�ķ���ֵ���ͣ�
 *          - #DDS_RETCODE_OK :��ʾ���óɹ���
 *          - #DDS_RETCODE_ERROR :��ʾʧ�ܣ����縴��QoSʱ��������
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_get_default_publisher_qos(
    DDS_DomainParticipant* self, 
    DDS_PublisherQos* qoslist);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_set_default_subscriber_qos( DDS_DomainParticipant* self, const DDS_SubscriberQos* qoslist);
 *
 * @ingroup CDomain
 *
 * @brief   �÷�������Ϊ�����߱����Ĭ��QoS���á�
 *
 * @details Ĭ�ϵ�QoS�ڴ����µĶ�����ʱָ��QoS����Ϊ #DDS_SUBSCRIBER_QOS_DEFAULT ʱʹ�õ�QoS���ã�
 *          ʹ�������ֵ #DDS_SUBSCRIBER_QOS_DEFAULT ������QoS�еĸ������õ�����ΪĬ��ֵ��
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

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_set_default_subscriber_qos(
    DDS_DomainParticipant* self, 
    const DDS_SubscriberQos* qoslist);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_get_default_subscriber_qos( DDS_DomainParticipant* self, DDS_SubscriberQos* qoslist);
 *
 * @ingroup CDomain
 *
 * @brief   �÷�����ȡΪ�����߱����Ĭ��QoS���á�
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in,out]  qoslist ���ڲ�����ʾ��ȡ�Ľ����
 *
 * @return  ��ǰ�ķ���ֵ���ͣ�
 *          - #DDS_RETCODE_OK :��ʾ���óɹ���
 *          - #DDS_RETCODE_ERROR :��ʾʧ�ܣ����縴��QoSʱ��������
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_get_default_subscriber_qos(
    DDS_DomainParticipant* self, 
    DDS_SubscriberQos* qoslist);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_set_default_topic_qos( DDS_DomainParticipant* self, const DDS_TopicQos* qoslist);
 *
 * @ingroup CDomain
 *
 * @brief   �÷�������Ϊ���Ᵽ���Ĭ��QoS���á�
 *
 * @details Ĭ�ϵ�QoS�ڴ����µ�����ʱָ��QoS����Ϊ #DDS_TOPIC_QOS_DEFAULT ʱʹ�õ�QoS���ã�
 *          ʹ�������ֵ #DDS_TOPIC_QOS_DEFAULT ����QoS�еĸ������õ�����ΪĬ��ֵ��
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

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_set_default_topic_qos(
    DDS_DomainParticipant* self, 
    const DDS_TopicQos* qoslist);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_get_default_topic_qos( DDS_DomainParticipant* self, DDS_TopicQos* qoslist);
 *
 * @ingroup CDomain
 *
 * @brief   �÷�����ȡΪ���Ᵽ���Ĭ��QoS���á�
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in,out]  qoslist ���ڲ�����ʾ��ȡ�Ľ��.
 *
 * @return  ��ǰ�ķ���ֵ���ͣ�
 *          - #DDS_RETCODE_OK :��ʾ���óɹ���
 *          - #DDS_RETCODE_ERROR :��ʾʧ�ܣ����縴��QoSʱ��������
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_get_default_topic_qos(
    DDS_DomainParticipant* self, 
    DDS_TopicQos* qoslist);

/**
 * @fn  DCPSDLL DDS_DomainId_t DDS_DomainParticipant_get_domain_id( DDS_DomainParticipant* self);
 *
 * @ingroup CDomain
 *
 * @brief   ��ȡ�����������������
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 *
 * @return  �������������ֵ��
 */

DCPSDLL DDS_DomainId_t DDS_DomainParticipant_get_domain_id(
    DDS_DomainParticipant* self);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_get_discovered_participants( DDS_DomainParticipant* self, DDS_InstanceHandleSeq* handles);
 *
 * @ingroup CDomain
 *
 * @brief   ��ȡ��������߷��ֵ�Զ�̣��߼��ϣ�Ҳ�п�����ͬһ���ڵ㣬������ͬһ��Ӧ�ó����У�������ߵı�ʶ�б�
 *
 * @details ��ZRDDS�У����������ʹ�ܺ󣬻�ͨ������Э���Զ���֪ͬһ�����µ�������ߣ�����֮������ǰʵ�����Ϣ��
 *          �Խ�������/���Ĺ�ϵ��������߷���Զ��������ߵ�����������
 *          - ������ͬһ�����ڣ�
 *          - δ���� #DDS_DomainParticipant_ignore_participant �ֶ�����Զ��������ߣ�
 *
 *          �û�����ͨ�����ַ�ʽ��ȡ��ǰ��������Ѿ����ֵ��������������Ϣ��
 *          - ͬ����ʽ���û�����Ҫ����Ϣʱ�����²����ȡ��
 *              - �û����ñ��ӿڻ�ȡ���ֵ�������߱�ʶ��
 *              - ���� #DDS_DomainParticipant_get_discovered_participant_data ͨ����һ���л�ȡ�ı�ʶ�鿴Զ��������ߵ���ϸ��Ϣ��
 *          - �첽�ص���ʽ
 *              - ͨ�������������ݶ��ߣ� DDS_ParticipantBuiltinTopicDataDataReader ���ļ�������
 *              - �ڼ������л�ȡ����Զ��������ߵ���ϸ��Ϣ��
 *          ͬ����ʽ�ŵ����ڱȽϼ򵥣�ȱ�����ڲ�ͬ��ʱ�Ļ�ȡ���µ�״̬������˵��Ҫ��ȡ��ʱ��״̬�Ĵ��۽ϸߣ�
 *          ͨ����Ƶ�ʵ���ѯ����
 *          �첽�ص���ʽ��ʹ����Ը��ӣ����ܹ���ʱ�Ļ�ȡ���µķ���״̬��
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in,out]  handles ���ڲ��������ڴ洢�ѷ��ֵ�������߱�ʶ�����û��ṩ�Ŀռ䲻��ʱ��
 *                  �ײ㽫���Զ����н������ݡ�
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK :��ʾ��ȡ�ɹ���
 *          - #DDS_RETCODE_NOT_ENABLED :��ǰ�������δʹ�ܣ�
 *          - #DDS_RETCODE_ERROR :��ȡʧ�ܣ�����ʧ�ܣ�
 *
 * @see DDS_DomainParticipant_get_discovered_participant_data
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_get_discovered_participants(
    DDS_DomainParticipant* self, 
    DDS_InstanceHandleSeq* handles);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_get_discovered_participant_data( DDS_DomainParticipant* self, DDS_ParticipantBuiltinTopicData* data, const DDS_InstanceHandle_t* handle);
 *
 * @ingroup CDomain
 *
 * @brief   �÷�����ѯָ����ʶ��������ߵ���ϸ��Ϣ��
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in,out]  data    ���ڲ��������ڴ洢��ȡ������ϸ��Ϣ��
 * @param   handle          ָ��������ߵ�Ψһ��ʶ���ñ�ʶ���Դ����¼����ط���ȡ��
 *                          - #DDS_DomainParticipant_get_discovered_participants
 *                          - �������ݶ����ж�ȡ�������������������е� DDS_ParticipantBuiltinTopicData::key
 *                              ���� DDS_SampleInfo::instance_handle
 *                          - Զ��������ߵ� #DDS_Entity_get_instance_handle �����Ľ����
 *
 * @return  - #DDS_RETCODE_OK :��ʾ���ڲ����е���ϸ��Ϣ��Ч������ȡ�ɹ���
 *          - #DDS_RETCODE_NOT_ENABLED :���������δʹ�ܣ�
 *          - #DDS_RETCODE_PRECONDITION_NOT_MET :�ṩ�ı�ʶ��Ч��
 *          - #DDS_RETCODE_ERROR :��ʾ��ȡʧ�ܣ����翽����������ʧ�ܣ�
 *
 * @see DDS_DomainParticipant_get_discovered_participants
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_get_discovered_participant_data(
    DDS_DomainParticipant* self, 
    DDS_ParticipantBuiltinTopicData* data, 
    const DDS_InstanceHandle_t* handle);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_get_discovered_topics( DDS_DomainParticipant* self, DDS_InstanceHandleSeq* handles);
 *
 * @ingroup CDomain
 *
 * @brief   ��ȡ�Ѿ���������û�б����Ե���������ı�ʶ��
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in,out]  handles ��ȡ�Ľ���б�
 *
 * @return  ��ǰ���Ƿ��� #DDS_RETCODE_UNSUPPORTED ��
 *
 * @warning ��ǰδʵ�֡�
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_get_discovered_topics(
    DDS_DomainParticipant* self, 
    DDS_InstanceHandleSeq* handles);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_get_discovered_topic_data( DDS_DomainParticipant* self, DDS_TopicBuiltinTopicData* data, const DDS_InstanceHandle_t* handle);
 *
 * @ingroup CDomain
 *
 * @brief   ��ȡ�Ѿ���������û�б����Ե��������ϸ��Ϣ��
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in,out]  data    ��ȡ������ϸ��Ϣ��
 * @param   handle          ��ʶ��Ҫ���ͻ�ȡ�����⡣
 *
 * @return  ��ǰ���Ƿ��� #DDS_RETCODE_UNSUPPORTED 
 *          
 * @warning ��ǰδʵ�֡�
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_get_discovered_topic_data(
    DDS_DomainParticipant* self, 
    DDS_TopicBuiltinTopicData* data, 
    const DDS_InstanceHandle_t* handle);

/**
 * @fn  DCPSDLL DDS_Boolean DDS_DomainParticipant_contains_entity( DDS_DomainParticipant* self, const DDS_InstanceHandle_t* handle);
 *
 * @ingroup CDomain
 *
 * @brief   �÷������ڲ���ʵ���Ƿ���������ߵ���ʵ�塣
 *
 * @details �÷�����ݹ���ԣ����ɲ��Ե�ʵ����������ߡ������ߡ����⡢���ݶ��ߡ�����д�ߡ�
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   handle    ��Ҫ���Ե�ʵ���ʶ��
 *
 * @return  true��ʾ���ڸ�������ߵ���ʵ�壬false��ʾ�����ڡ�
 */

DCPSDLL DDS_Boolean DDS_DomainParticipant_contains_entity(
    DDS_DomainParticipant* self,
    const DDS_InstanceHandle_t* handle);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_get_current_time( DDS_DomainParticipant* self, DDS_Time_t* currentTime);
 *
 * @ingroup CDomain
 *
 * @brief   ��ȡZRDDS�ڲ�ʹ�õ�ʱ��ϵͳ�ĵ�ǰʱ�����
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in,out]  currentTime    ���ڲ�������ǰ��ʱ�����
 *
 * @return  ��ǰ���Ƿ��� #DDS_RETCODE_OK ��
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_get_current_time(
    DDS_DomainParticipant* self, 
    DDS_Time_t* currentTime);

/**
 * @fn  DCPSDLL DDS_Entity* DDS_DomainParticipant_as_entity( DDS_DomainParticipant *self);
 *
 * @ingroup CDomain
 *
 * @brief   ���������ת��Ϊ�����ࡱʵ�����
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 *
 * @return  �ձ�ʾת��ʧ�ܣ�����ָ�򡰸��ࡱʵ�����
 */

DCPSDLL DDS_Entity* DDS_DomainParticipant_as_entity(
    DDS_DomainParticipant *self);

#ifdef _ZRXMLINTERFACE

#ifdef _ZRXMLENTITYINTERFACE
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_register_type_from_type_library(
    DDS_DomainParticipant* self,
    const DDS_Char* type_name,
    const DDS_Char* registered_name);

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_unregister_type_from_type_library(
    DDS_DomainParticipant* self,
    const DDS_Char* registered_name);

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_lookup_named_publishers(
    DDS_DomainParticipant* self,
    const char* pattern, DDS_StringSeq* publisher_names);

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_lookup_named_subscribers(
    DDS_DomainParticipant* self,
    const char* pattern, DDS_StringSeq* subscriber_names);

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_lookup_named_topics(
    DDS_DomainParticipant* self,
    const char* pattern, DDS_StringSeq* topic_names);

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_register_type_from_xml_string(
    DDS_DomainParticipant* self, const DDS_Char* xml_content);

DCPSDLL const DDS_Char* DDS_DomainParticipant_get_entity_name(
    DDS_DomainParticipant* self);

DCPSDLL DDS_DomainParticipantFactory* DDS_DomainParticipant_get_factory(
    DDS_DomainParticipant* self);

DCPSDLL DDS_Publisher* DDS_DomainParticipant_lookup_publisher_by_name(
    DDS_DomainParticipant* self,
    const DDS_Char* name);

DCPSDLL DDS_Subscriber* DDS_DomainParticipant_lookup_subscriber_by_name(
    DDS_DomainParticipant* self,
    const DDS_Char* name);

DCPSDLL DDS_Topic* DDS_DomainParticipant_lookup_topic_by_name(
    DDS_DomainParticipant* self,
    const DDS_Char* name);

DCPSDLL DDS_Publisher* DDS_DomainParticipant_create_publisher_from_xml_string(
    DDS_DomainParticipant* self, const DDS_Char* xml_content);

DCPSDLL DDS_Subscriber* DDS_DomainParticipant_create_subscriber_from_xml_string(
    DDS_DomainParticipant* self, const DDS_Char* xml_content);

DCPSDLL DDS_Topic* DDS_DomainParticipant_create_topic_from_xml_string(
    DDS_DomainParticipant* self, const DDS_Char* xml_content);

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_to_xml(
    DDS_DomainParticipant* self, const DDS_Char** result, DDS_Boolean contained_qos);

#endif /* _ZRXMLENTITYINTERFACE */

#ifdef _ZRXMLQOSINTERFACE

/**
 * @fn  DCPSDLL DDS_Publisher* DDS_DomainParticipant_create_publisher_with_qos_profile( DDS_DomainParticipant* self, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name, DDS_PublisherListener* listener, DDS_StatusKindMask mask);
 *
 * @ingroup CDomain
 *
 * @brief   ��QoS�ֿ��л�ȡ������Qos�����䴴�������ߡ�
 *
 * @param [in,out]  self        ָ��Ŀ��
 * @param   library_name        QoS������֣�������ΪNULL��
 * @param   profile_name        QoS���õ����֣�������ΪNULL��
 * @param   qos_name            QoS�����֣�����ΪNULL����ת��Ϊdefault�ַ�����
 * @param [in,out]  listener    Ϊ�÷��������õļ�������
 * @param   mask                ����Ӧ�ó������Ȥ��״̬��ֻ��Ӧ�ó������Ȥ��״̬�����仯ʱ���Ż�֪ͨӦ�ó���
 *
 * @return  �����ɹ�ָ�򴴽��ɹ��ķ�����ʵ����󣬷��򷵻�NULL��ʧ�ܵ�ԭ�����Ϊ��
 *          - ����ռ�ʧ�ܻ��߳�ʼ����Դʧ�ܣ�����Ĵ�����Ϣ�μ���־��
 *          - δ�ҵ�ָ����QoS�ȡ�
 */
DCPSDLL DDS_Publisher* DDS_DomainParticipant_create_publisher_with_qos_profile(
    DDS_DomainParticipant* self,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name,
    DDS_PublisherListener* listener,
    DDS_StatusKindMask mask);

/**
 * @fn  DCPSDLL DDS_Subscriber* DDS_DomainParticipant_create_subscriber_with_qos_profile( DDS_DomainParticipant* self, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name, DDS_SubscriberListener* listener, DDS_StatusKindMask mask);
 *
 * @ingroup CDomain
 *
 * @brief   ��QoS�ֿ��л�ȡ������Qos�����䴴��������
 *
 * @param [in,out]  self        ָ��Ŀ��
 * @param   library_name        QoS������֣�������ΪNULL��
 * @param   profile_name        QoS���õ����֣�������ΪNULL��
 * @param   qos_name            QoS�����֣�����ΪNULL����ת��Ϊdefault�ַ�����
 * @param [in,out]  listener    Ϊ�ö��������õļ�������
 * @param   mask                ����Ӧ�ó������Ȥ��״̬��ֻ��Ӧ�ó������Ȥ��״̬�����仯ʱ���Ż�֪ͨӦ�ó���
 *
 * @return  �����ɹ�ָ�򴴽��ɹ��Ķ�����ʵ����󣬷��򷵻�NULL��ʧ�ܵ�ԭ�����Ϊ��
 *          - ����ռ�ʧ�ܻ��߳�ʼ����Դʧ�ܣ�����Ĵ�����Ϣ�μ���־��
 *          - δ�ҵ�ָ����QoS�ȡ�
 */
DCPSDLL DDS_Subscriber* DDS_DomainParticipant_create_subscriber_with_qos_profile(
    DDS_DomainParticipant* self,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name,
    DDS_SubscriberListener* listener,
    DDS_StatusKindMask mask);

/**
 * @fn  DCPSDLL DDS_Topic* DDS_DomainParticipant_create_topic_with_qos_profile( DDS_DomainParticipant* self, const DDS_Char* topic_name, const DDS_Char* type_name, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name, DDS_TopicListener* listener, DDS_StatusKindMask mask);
 *
 * @ingroup CDomain
 *
 * @brief   ��QoS�ֿ��л�ȡ����Qos�����䴴�����⡣
 *
 * @param [in,out]  self        ָ��Ŀ��
 * @param   topic_name          ���������
 * @param   type_name           ��������������
 * @param   library_name        QoS������֣�������ΪNULL��
 * @param   profile_name        QoS���õ����֣�������ΪNULL��
 * @param   qos_name            QoS�����֣�����ΪNULL����ת��Ϊdefault�ַ�����
 * @param [in,out]  listener    Ϊ���������õļ�������
 * @param   mask                ����Ӧ�ó������Ȥ��״̬��ֻ��Ӧ�ó������Ȥ��״̬�����仯ʱ���Ż�֪ͨӦ�ó���
 *
 * @return  �����ɹ�ָ�򴴽��ɹ�������ʵ����󣬷��򷵻�NULL��ʧ�ܵ�ԭ�����Ϊ��
 *          - ����ռ�ʧ�ܻ��߳�ʼ����Դʧ�ܣ�����Ĵ�����Ϣ�μ���־��
 *          - δ�ҵ�ָ����QoS�ȡ�
 */
DCPSDLL DDS_Topic* DDS_DomainParticipant_create_topic_with_qos_profile(
    DDS_DomainParticipant* self,
    const DDS_Char* topic_name,
    const DDS_Char* type_name,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name,
    DDS_TopicListener* listener,
    DDS_StatusKindMask mask);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_set_default_publisher_qos_with_profile( DDS_DomainParticipant* self, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name);
 *
 * @ingroup CDomain
 *
 * @brief   ��QoS�ֿ��ȡ������QoS��������ΪĬ�Ϸ�����Qos
 *
 * @param [in,out]  self    ָ��Ŀ��
 * @param   library_name    QoS������֣�������ΪNULL��
 * @param   profile_name    QoS���õ����֣�������ΪNULL��
 * @param   qos_name        QoS�����֣�����ΪNULL����ת��Ϊdefault�ַ�����
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *          - #DDS_RETCODE_OK :��ʾ���óɹ���
 *          - #DDS_RETCODE_ERROR :��ʾδ֪�����µ����ô���
 */
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_set_default_publisher_qos_with_profile(
    DDS_DomainParticipant* self,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_set_default_subscriber_qos_with_profile( DDS_DomainParticipant* self, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name);
 *
 * @ingroup CDomain
 *
 * @brief   ��QoS�ֿ��ȡ������QoS��������ΪĬ�϶�����Qos
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
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_set_default_subscriber_qos_with_profile(
    DDS_DomainParticipant* self,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_set_default_topic_qos_with_profile( DDS_DomainParticipant* self, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name);
 *
 * @ingroup CDomain
 *
 * @brief   ��QoS�ֿ��ȡ����QoS��������ΪĬ������Qos
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
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_set_default_topic_qos_with_profile(
    DDS_DomainParticipant* self,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_set_qos_with_profile( DDS_DomainParticipant* self, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name);
 *
 * @ingroup CDomain
 *
 * @brief   ��QoS�ֿ��ȡQoS���ò����õ��������
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
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipant_set_qos_with_profile(
    DDS_DomainParticipant* self,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name);

#endif /* _ZRXMLQOSINTERFACE */

#endif /* _ZRXMLINTERFACE */

#ifdef __cplusplus
}
#endif

#endif /* DDS_DomainParticipant_h__*/
