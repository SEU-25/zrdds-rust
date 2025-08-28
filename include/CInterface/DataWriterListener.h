/**
 * @file:       DataWriterListener.h
 *
 * copyright:   Copyright (c) 2018 ZhenRong Technology, Inc. All rights reserved.
 */

#ifndef DataWriterListener_h__
#define DataWriterListener_h__

#include "Listener.h"
#include "OfferedDeadlineMissedStatus.h"
#include "LivelinessLostStatus.h"
#include "OfferedIncompatibleQosStatus.h"
#include "PublicationMatchedStatus.h"
#include "ZRDDSCWrapper.h"

#ifdef _ZRDDS_INCLUDE_DEADLINE_QOS

/**
 * @typedef void(*DataWriterListenerOfferedDeadlineMissedCallback)( DDS_DataWriter* writer, const DDS_OfferedDeadlineMissedStatus* status)
 *
 * @ingroup CListener
 *
 * @brief �ײ� #DDS_OFFERED_DEADLINE_MISSED_STATUS ״̬�Ļص���������ָ�����͡�
 *
 * @param [in,out] writer �ü���������������д�ߡ�
 * @param status              ��ǰ��״ֵ̬��
 */

typedef void(*DataWriterListenerOfferedDeadlineMissedCallback)(
    DDS_DataWriter* writer,
    const DDS_OfferedDeadlineMissedStatus* status);
#endif /* _ZRDDS_INCLUDE_DEADLINE_QOS */

#ifdef _ZRDDS_INCLUDE_LIVELINESS_QOS

/**
 * @typedef void(*DataWriterListenerLivelinessLostCallback)( DDS_DataWriter* writer, const DDS_LivelinessLostStatus* status)
 *
 * @ingroup CListener
 *
 * @brief �ײ� #DDS_LIVELINESS_LOST_STATUS ״̬�Ļص���������ָ�����͡�
 *
 * @param [in,out] writer �ü���������������д�ߡ�
 * @param status              ��ǰ��״ֵ̬��
 */

typedef void(*DataWriterListenerLivelinessLostCallback)(
    DDS_DataWriter* writer,
    const DDS_LivelinessLostStatus* status);
#endif /* _ZRDDS_INCLUDE_LIVELINESS_QOS */

/**
 * @typedef void(*DataWriterListenerOfferedIncompatibleQosCallback)( DDS_DataWriter* writer, const DDS_OfferedIncompatibleQosStatus* status)
 *
 * @ingroup CListener
 *
 * @brief �ײ� #DDS_OFFERED_INCOMPATIBLE_QOS_STATUS ״̬�Ļص���������ָ�����͡�
 *
 * @param [in,out] writer �ü���������������д�ߡ�
 * @param status              ��ǰ��״ֵ̬��
 */

typedef void(*DataWriterListenerOfferedIncompatibleQosCallback)(
    DDS_DataWriter* writer,
    const DDS_OfferedIncompatibleQosStatus* status);

/**
 * @typedef void(*DataWriterListenerPublicationMatchedCallback)( DDS_DataWriter* writer, const DDS_PublicationMatchedStatus* status)
 *
 * @ingroup CListener
 *
 * @brief �ײ� #DDS_PUBLICATION_MATCHED_STATUS ״̬�Ļص���������ָ�����͡�
 *
 * @param [in,out] writer �ü���������������д�ߡ�
 * @param status              ��ǰ��״ֵ̬��
 */

typedef void(*DataWriterListenerPublicationMatchedCallback)(
    DDS_DataWriter* writer,
    const DDS_PublicationMatchedStatus* status);

/**
 * @struct DDS_DataWriterListener
 *
 * @ingroup CListener
 *
 * @brief   ����д�߼��������͡�
 */

typedef struct DDS_DataWriterListener
{
    /** @brief   "����"���� */
    DDS_Listener listener;
#ifdef _ZRDDS_INCLUDE_DEADLINE_QOS
    /** @brief  Ϊ #DDS_OFFERED_DEADLINE_MISSED_STATUS ״̬���õĻص������� */
    DataWriterListenerOfferedDeadlineMissedCallback on_offered_deadline_missed;
#endif /* _ZRDDS_INCLUDE_DEADLINE_QOS */
    /** @brief  Ϊ #DDS_OFFERED_INCOMPATIBLE_QOS_STATUS ״̬���õĻص������� */
    DataWriterListenerOfferedIncompatibleQosCallback on_offered_incompatible_qos;
#ifdef _ZRDDS_INCLUDE_LIVELINESS_QOS
    /** @brief  Ϊ #DDS_LIVELINESS_LOST_STATUS ״̬���õĻص������� */
    DataWriterListenerLivelinessLostCallback on_liveliness_lost;
#endif /* _ZRDDS_INCLUDE_LIVELINESS_QOS */
    /** @brief  Ϊ #DDS_PUBLICATION_MATCHED_STATUS ״̬���õĻص������� */
    DataWriterListenerPublicationMatchedCallback on_publication_matched;
}DDS_DataWriterListener;

/**
 * @def DDS_DataWriterListener_initial(listener)
 *
 * @ingroup CListener
 *
 * @brief   ��ʼ������д�߼���������Ϊ���г�ԱΪ�ա�
 *
 * @param   listener    ��Ҫ��ʼ��������д�߼���������
 */
#define DDS_DataWriterListener_initial(listener) memset(listener, 0, sizeof(DDS_DataWriterListener))

#endif /* DataWriterListener_h__*/
