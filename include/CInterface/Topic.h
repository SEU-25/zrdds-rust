/**
 * @file:       Topic.h
 *
 * copyright:   Copyright (c) 2018 ZhenRong Technology, Inc. All rights reserved.
 */

#ifndef TopicDescription_h__
#define TopicDescription_h__

#include "InconsistentTopicStatus.h"
#include "ReturnCode_t.h"
#include "TopicQos.h"
#include "TopicListener.h"
#include "StatusKindMask.h"
#include "Entity.h"

#ifdef __cplusplus
extern "C"
{
#endif

/** @brief	�������������Ϣ���ݶ��߹������������ơ�@ingroup   CoreVar */
DCPSDLL extern const DDS_Char* BUILTIN_PARTICIPANT_TOPIC_NAME;
/** @brief	��������д������Ϣ���ݶ��߹������������ơ�@ingroup   CoreVar */
DCPSDLL extern const DDS_Char* BUILTIN_PUBLICATION_TOPIC_NAME;
/** @brief	�������ݶ�����Ϣ���ݶ��߹������������ơ�@ingroup   CoreVar */
DCPSDLL extern const DDS_Char* BUILTIN_SUBSCRIPTION_TOPIC_NAME;

/**
 * @fn  DCPSDLL const DDS_Char* DDS_TopicDescription_get_name( const DDS_TopicDescription* self);
 *
 * @ingroup CTopic
 *
 * @brief   ��ȡ����������ơ�
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 *
 * @return  �������ơ�
 */

DCPSDLL const DDS_Char* DDS_TopicDescription_get_name(
    const DDS_TopicDescription* self);

/**
 * @fn  DCPSDLL const DDS_Char* DDS_TopicDescription_get_type_name( const DDS_TopicDescription* self);
 *
 * @ingroup CTopic
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ  
 *  
 * @brief   ��ȡ�������������������������������ע������ơ�
 *
 * @return  �������͵����ơ�
 */

DCPSDLL const DDS_Char* DDS_TopicDescription_get_type_name(
    const DDS_TopicDescription* self);

/**
 * @fn  DCPSDLL DDS_DomainParticipant* DDS_TopicDescription_get_participant( const DDS_TopicDescription* self);
 *
 * @ingroup CTopic
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 *                  
 * @brief   ��ȡ����������������������ߡ�
 *
 * @return  ���ظ�����������������߶���
 */

DCPSDLL DDS_DomainParticipant* DDS_TopicDescription_get_participant(
    const DDS_TopicDescription* self);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Topic_get_inconsistent_topic_status( DDS_Topic* self, DDS_InconsistentTopicStatus* status);
 *
 * @ingroup CTopic
 *
 * @brief   ��ȡ����������� #DDS_INCONSISTENT_TOPIC_STATUS ״̬.
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in,out]  status  ���ڲ�����ʾ��ǰ��״̬��
 *
 * @return  ��ǰ���Ƿ��� #DDS_RETCODE_OK ��ʾ��ȡ�ɹ���
 */

DCPSDLL DDS_ReturnCode_t DDS_Topic_get_inconsistent_topic_status(
    DDS_Topic* self, 
    DDS_InconsistentTopicStatus* status);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Topic_set_qos( DDS_Topic* self, const DDS_TopicQos* qoslist);
 *
 * @ingroup CTopic
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   qoslist ��ʾ�û���Ҫ���õ�QoS���á�
 *
 * @details ����ʹ������ֵ #DDS_TOPIC_QOS_DEFAULT ��
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK :��ʾ��ȡ�ɹ���
 *          - #DDS_RETCODE_BAD_PARAMETER :��ʾ @e qoslist ������Ч��QoS���ã�
 *          - #DDS_RETCODE_INCONSISTENT :��ʾ @e qoslist ���в����ݵ�QoS���ã�
 *          - #DDS_RETCODE_IMMUTABLE_POLICY :��ʾ�û������޸�ʹ�ܺ󲻿ɱ��QoS���ã�
 *          - #DDS_RETCODE_ERROR :��ʾδ����Ĵ��󣬴�����ϸ��Ϣ�������־�У�
 */

DCPSDLL DDS_ReturnCode_t DDS_Topic_set_qos(
    DDS_Topic* self, 
    const DDS_TopicQos* qoslist);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Topic_get_qos( DDS_Topic* self, DDS_TopicQos* qos);
 *
 * @ingroup CTopic
 *
 * @brief   ��ȡ�������QoS���á�
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in,out]  qos ���ڲ��������ڱ��������ĵ�QoS���á�
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK :��ʾ��ȡ�ɹ���
 *          - #DDS_RETCODE_ERROR :��ʾʧ�ܣ�ԭ�����Ϊ����QoSʱ��������
 */

DCPSDLL DDS_ReturnCode_t DDS_Topic_get_qos(
    DDS_Topic* self, 
    DDS_TopicQos* qos);

/**
 * @fn  DCPSDLL DDS_TopicListener* DDS_Topic_get_listener( DDS_Topic* self);
 *
 * @ingroup CTopic
 *
 * @brief   �÷�����ȡͨ�� #DDS_Topic_set_listener �������ߴ���ʱΪ�������õļ���������
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 *                  
 * @return  ��ǰ���ܵķ���ֵ��
 *          - NULL��ʾδ���ü�������
 *          - �ǿձ�ʾӦ��ͨ�� #DDS_Topic_set_listener �����ڴ���ʱ���õļ���������
 */

DCPSDLL DDS_TopicListener* DDS_Topic_get_listener(
    DDS_Topic* self);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Topic_set_listener( DDS_Topic* self, DDS_TopicListener* listener, DDS_StatusKindMask mask);
 *
 * @ingroup CTopic
 *
 * @brief   ���ø�����ĵļ�������
 *
 * @details  ������������ԭ�м�������������ÿն����ʾ���ԭ�����õļ�������
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 *                  
 * @param [in,out]  listener  Ϊ���������õļ���������
 * @param   mask                ����Ӧ�ó������Ȥ��״̬��ֻ��Ӧ�ó������Ȥ��״̬�����仯ʱ���Ż�֪ͨӦ�ó���
 *
 * @return  ��ǰ���Ƿ��� #DDS_RETCODE_OK ��ʾ���óɹ���
 */

DCPSDLL DDS_ReturnCode_t DDS_Topic_set_listener(
    DDS_Topic* self,
    DDS_TopicListener* listener,
    DDS_StatusKindMask mask);

/**
 * @fn  DCPSDLL DDS_Entity* DDS_Topic_as_entity(DDS_Topic* topic);
 *
 * @ingroup CTopic
 *
 * @brief   ������ת��Ϊ�����ࡱʵ�����
 *
 * @param [in,out]  topic    ָ��Ŀ�ꡣ
 *
 * @return  �ձ�ʾת��ʧ�ܣ�����ָ�򡰸��ࡱʵ�����
 */

DCPSDLL DDS_Entity* DDS_Topic_as_entity(DDS_Topic* topic);

#ifdef _ZRXMLINTERFACE

#ifdef _ZRXMLENTITYINTERFACE

DCPSDLL DDS_ReturnCode_t DDS_Topic_to_xml(
    DDS_Topic* self,
    const DDS_Char** result,
    DDS_Boolean contained_qos);

DCPSDLL const DDS_Char* DDS_Topic_get_entity_name(
    DDS_Topic* self);

DCPSDLL DDS_DomainParticipant* DDS_Topic_get_factory(
    DDS_Topic* self);

#endif /* _ZRXMLENTITYINTERFACE */

#ifdef _ZRXMLQOSINTERFACE

/**
 * @fn  DCPSDLL DDS_DomainParticipant* DDS_Topic_set_qos_with_profile( DDS_Topic* self, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name);
 *
 * @ingroup CTopic
 *
 * @brief   ��QoS�ֿ��л�ȡ����QoS���������õ�������
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
DCPSDLL DDS_ReturnCode_t DDS_Topic_set_qos_with_profile(
    DDS_Topic* self,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name);
#endif /* _ZRXMLQOSINTERFACE */

#endif /* _ZRXMLINTERFACE */

#ifdef __cplusplus
}
#endif

#endif /* TopicDescription_h__*/
