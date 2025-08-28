/**
 * @file:       BuiltinDataDataReader.h
 *
 * copyright:   Copyright (c) 2018 ZhenRong Technology, Inc. All rights reserved.
 */

#ifndef BuiltinDataDataReader_h__
#define BuiltinDataDataReader_h__

#include "ZRDDSDataReader.h"
#include "ParticipantBuiltinTopicData.h"
#include "PublicationBuiltinTopicData.h"
#include "SubscriptionBuiltinTopicData.h"
#include "TopicBuiltinTopicData.h"

#ifdef __cplusplus
extern "C"
{
#endif

/**
 * @struct DDS_ParticipantBuiltinTopicDataSeq
 *
 * @ingroup CDiscoveryTypes
 *
 * @brief   ���������������� DDS_ParticipantBuiltinTopicData �����С�
 */

/**  
 * @class DDS_ParticipantBuiltinTopicDataDataReader
 *
 * @ingroup CDiscoveryTypes
 *
 * @brief ���������������� DDS_ParticipantBuiltinTopicData ���������ݶ���
 *  
 * @details ���ڴ�ZRDDS�л�ȡ������������������ݼ���״̬�������� ZRDDSDataReader ��ʵ������ģ�����Ϊ�� 
 *          DDS_ParticipantBuiltinTopicData DDS_ParticipantBuiltinTopicDataSeq ���÷��μ� 
 *          #DDS_DomainParticipant_get_builtin_subscriber ��
 */

typedef struct DDS_ParticipantBuiltinTopicDataDataReader DDS_ParticipantBuiltinTopicDataDataReader;
ZRDDSDataReader(DDS_ParticipantBuiltinTopicDataDataReader, DDS_ParticipantBuiltinTopicDataSeq, DDS_ParticipantBuiltinTopicData);

/**
 * @struct DDS_PublicationBuiltinTopicDataSeq
 *
 * @ingroup CDiscoveryTypes
 *
 * @brief   ���������������� DDS_PublicationBuiltinTopicData �����С�
 */

/**  
 * @class DDS_PublicationBuiltinTopicDataDataReader
 *
 * @ingroup CDiscoveryTypes
 *
 * @brief ���������������� DDS_PublicationBuiltinTopicData ���������ݶ���
 *  
 * @details ���ڴ�ZRDDS�л�ȡ������������������ݼ���״̬�������� ZRDDSDataReader ��ʵ������ģ�����Ϊ�� 
 *          DDS_PublicationBuiltinTopicData DDS_PublicationBuiltinTopicDataSeq ���÷��μ� 
 *          #DDS_DomainParticipant_get_builtin_subscriber ��
 */

typedef struct DDS_PublicationBuiltinTopicDataDataReader DDS_PublicationBuiltinTopicDataDataReader;
ZRDDSDataReader(DDS_PublicationBuiltinTopicDataDataReader, DDS_PublicationBuiltinTopicDataSeq, DDS_PublicationBuiltinTopicData);

/**
 * @struct DDS_PublicationBuiltinTopicDataSeq
 *
 * @ingroup CDiscoveryTypes
 *
 * @brief   ���������������� DDS_PublicationBuiltinTopicData �����С�
 */

/**  
 * @class DDS_SubscriptionBuiltinTopicDataDataReader
 *
 * @ingroup CDiscoveryTypes
 *
 * @brief ���������������� DDS_SubscriptionBuiltinTopicData ���������ݶ���
 *  
 * @details ���ڴ�ZRDDS�л�ȡ������������������ݼ���״̬�������� ZRDDSDataReader ��ʵ������ģ�����Ϊ�� 
 *          DDS_SubscriptionBuiltinTopicData DDS_SubscriptionBuiltinTopicDataSeq ���÷��μ� 
 *          #DDS_DomainParticipant_get_builtin_subscriber ��
 */

typedef struct DDS_SubscriptionBuiltinTopicDataDataReader DDS_SubscriptionBuiltinTopicDataDataReader;
ZRDDSDataReader(DDS_SubscriptionBuiltinTopicDataDataReader, DDS_SubscriptionBuiltinTopicDataSeq, DDS_SubscriptionBuiltinTopicData);

#ifdef _ZRDDS_INCLUDE_TOPIC_BUILTIN_TOPIC_DATA


/**  
 * @class DDS_TopicBuiltinTopicDataDataReader
 *
 * @ingroup CDiscoveryTypes
 *
 * @brief ���������������� DDS_TopicBuiltinTopicData ���������ݶ���
 *  
 * @details ���ڴ�ZRDDS�л�ȡ���ö��Ķ��������ݣ������� ZRDDSDataReader ��ʵ������ģ�����Ϊ��
 *          DDS_TopicBuiltinTopicData DDS_TopicBuiltinTopicDataSeq ���÷��μ�
 *          #DDS_DomainParticipant_get_builtin_subscriber ��
 */
typedef struct DDS_TopicBuiltinTopicDataDataReader DDS_TopicBuiltinTopicDataDataReader;
ZRDDSDataReader(DDS_TopicBuiltinTopicDataDataReader, DDS_TopicBuiltinTopicDataSeq, DDS_TopicBuiltinTopicData);

#endif /* _ZRDDS_INCLUDE_TOPIC_BUILTIN_TOPIC_DATA */

#ifdef __cplusplus
}
#endif

#endif /* BuiltinDataDataReader_h__*/
