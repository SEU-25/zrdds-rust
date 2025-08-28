/**
 * @file:       DataReaderListener.h
 *
 * copyright:   Copyright (c) 2018 ZhenRong Technology, Inc. All rights reserved.
 */

#ifndef DataReaderListener_h__
#define DataReaderListener_h__

#include "Listener.h"
#include "SampleInfo.h"
#include "RequestedDeadlineMissedStatus.h"
#include "LivelinessChangedStatus.h"
#include "RequestedIncompatibleQosStatus.h"
#include "SampleRejectedStatus.h"
#include "SubscriptionMatchedStatus.h"
#include "SampleLostStatus.h"
#include "ZRDDSCWrapper.h"
#include <stdio.h>

#ifdef _ZRDDS_INCLUDE_DEADLINE_QOS

/**
 * @typedef void(*DataReaderListenerRequestedDeadlineMissedCallback)( DDS_DataReader* reader, const DDS_RequestedDeadlineMissedStatus* status)
 *
 * @ingroup CListener
 *
 * @brief   �ײ� #DDS_REQUESTED_DEADLINE_MISSED_STATUS ״̬�Ļص���������ָ�����͡�
 *
 * @param [in,out]  reader  �ü��������������ݶ��ߡ�
 * @param   status          ��ǰ��״ֵ̬��
 */

typedef void(*DataReaderListenerRequestedDeadlineMissedCallback)(
    DDS_DataReader* reader,
    const DDS_RequestedDeadlineMissedStatus* status);
#endif /* _ZRDDS_INCLUDE_DEADLINE_QOS */

#ifdef _ZRDDS_INCLUDE_LIVELINESS_QOS

/**
 * @typedef void(*DataReaderListenerLivelinessChangedCallback)( DDS_DataReader* reader, const DDS_LivelinessChangedStatus* status)
 *
 * @ingroup CListener
 *
 * @brief   �ײ� #DDS_LIVELINESS_LOST_STATUS ״̬�Ļص���������ָ�����͡�
 *
 * @param [in,out]  reader  �ü��������������ݶ��ߡ�
 * @param   status          ��ǰ��״ֵ̬��
 */

typedef void(*DataReaderListenerLivelinessChangedCallback)(
    DDS_DataReader* reader,
    const DDS_LivelinessChangedStatus* status);
#endif /* _ZRDDS_INCLUDE_LIVELINESS_QOS */

/**
 * @typedef void(*DataReaderListenerRequestedIncompatibleQosCallback)( DDS_DataReader* reader, const DDS_RequestedIncompatibleQosStatus* status)
 *
 * @ingroup CListener
 *
 * @brief   �ײ� #DDS_REQUESTED_INCOMPATIBLE_QOS_STATUS ״̬�Ļص���������ָ�����͡�
 *
 * @param [in,out]  reader  �ü��������������ݶ��ߡ�
 * @param   status          ��ǰ��״ֵ̬��
 */

typedef void(*DataReaderListenerRequestedIncompatibleQosCallback)(
    DDS_DataReader* reader,
    const DDS_RequestedIncompatibleQosStatus* status);

/**
 * @typedef void(*DataReaderListenerSampleRejectedCallback)( DDS_DataReader* reader, const DDS_SampleRejectedStatus* status)
 *
 * @ingroup CListener
 *
 * @brief   �ײ� #DDS_SAMPLE_REJECTED_STATUS ״̬�Ļص���������ָ�����͡�
 *
 * @param [in,out]  reader  �ü��������������ݶ��ߡ�
 * @param   status          ��ǰ��״ֵ̬��
 */

typedef void(*DataReaderListenerSampleRejectedCallback)(
    DDS_DataReader* reader,
    const DDS_SampleRejectedStatus* status);

/**
 * @typedef void(*DataReaderListenerDataAvailableCallback)( DDS_DataReader* reader)
 *
 * @ingroup CListener
 *
 * @brief   ���µ������洢�����ݶ��ߵײ�ʱ�ص���
 *
 * @details �ûص��������µ��������������ݣ��û���Ҫͨ�� @ref read-take ϵ�а���Ҫ���ȡ�������������ݡ�
 *
 * @param [in,out]  reader  �ü��������������ݶ��ߡ�
 */

typedef void(*DataReaderListenerDataAvailableCallback)(
    DDS_DataReader* reader);

/**
 * @typedef void(*DataReaderListenerSubscriptionMatchedCallback)( DDS_DataReader* reader, const DDS_SubscriptionMatchedStatus* status)
 *
 * @ingroup CListener
 *
 * @brief   �ײ� #DDS_SUBSCRIPTION_MATCHED_STATUS ״̬�Ļص���������ָ�����͡�
 *
 * @param [in,out]  reader  �ü��������������ݶ��ߡ�
 * @param   status          ��ǰ��״ֵ̬��
 */

typedef void(*DataReaderListenerSubscriptionMatchedCallback)(
    DDS_DataReader* reader,
    const DDS_SubscriptionMatchedStatus* status);

/**
 * @typedef void(*DataReaderListenerSampleLostCallback)( DDS_DataReader* reader, const DDS_SampleLostStatus* status)
 *
 * @ingroup CListener
 *
 * @brief   �ײ� #DDS_SAMPLE_LOST_STATUS ״̬�Ļص���������ָ�����͡�
 *
 * @param [in,out]  reader  �ü��������������ݶ��ߡ�
 * @param   status          ��ǰ��״ֵ̬��
 */

typedef void(*DataReaderListenerSampleLostCallback)(
    DDS_DataReader* reader,
    const DDS_SampleLostStatus* status);

typedef void(*DataReaderListenerSampleArrivedCallback)(
    DDS_DataReader* reader,
    void* sample,
    const DDS_SampleInfo* sampleInfo);

/**
 * @struct DDS_DataReaderListener
 *
 * @ingroup  CListener
 *
 * @brief   ���ݶ��߼��������͡�
 */

typedef struct DDS_DataReaderListener
{
    /** @brief   ��������� */
    DDS_Listener listener;
#ifdef _ZRDDS_INCLUDE_DEADLINE_QOS
    /** @brief  Ϊ #DDS_REQUESTED_DEADLINE_MISSED_STATUS ״̬���õĻص������� */
    DataReaderListenerRequestedDeadlineMissedCallback
        on_requested_deadline_missed;
#endif /* _ZRDDS_INCLUDE_DEADLINE_QOS */
    /** @brief  Ϊ #DDS_REQUESTED_INCOMPATIBLE_QOS_STATUS ״̬���õĻص������� */
    DataReaderListenerRequestedIncompatibleQosCallback
        on_requested_incompatible_qos;
    /** @brief  Ϊ #DDS_SAMPLE_REJECTED_STATUS ״̬���õĻص������� */
    DataReaderListenerSampleRejectedCallback 
        on_sample_rejected;
    /** @brief  Ϊ #DDS_SAMPLE_LOST_STATUS ״̬���õĻص������� */
    DataReaderListenerSampleLostCallback
        on_sample_lost;
#ifdef _ZRDDS_INCLUDE_LIVELINESS_QOS
    /** @brief  Ϊ #DDS_LIVELINESS_LOST_STATUS ״̬���õĻص������� */
    DataReaderListenerLivelinessChangedCallback
        on_liveliness_changed;
#endif /* _ZRDDS_INCLUDE_LIVELINESS_QOS */
    /** @brief  ���µ������洢�����ݶ��ߵײ�ʱ���õĻص������� */
    DataReaderListenerDataAvailableCallback 
        on_data_available;
    DataReaderListenerSampleArrivedCallback
        on_data_arrived;
    /** @brief  Ϊ #DDS_SUBSCRIPTION_MATCHED_STATUS ״̬���õĻص������� */
    DataReaderListenerSubscriptionMatchedCallback 
        on_subscription_matched;
} DDS_DataReaderListener;

/**
 * @def DDS_SimpleDataReaderListener(NAME, TYPE)
 *
 * @ingroup  CListener
 *
 * @brief   ���ڼ����ݶ��߼����������ݵ��ﺯ���Ĵ��븴�Ӷȣ��ú꽫������¹�����
 *          - ���� NAME_on_data_available �������÷�����DDS�ײ������ݶ��������ݵ���ʱ���ã���ִ�дӵײ��ȡ�����µ����ݲ����ε��� NAME_on_process_sample ������
 *          - �����˺��� NAME_on_process �ķ���ֵ�Լ���������
 *
 *          ������ͬ���ͣ������ǲ�ͬ�����⣩����ʹ��ͬһ���ص�������
 *          ʹ�� `const DDS_Char* topicName = DDS_TopicDescription_get_name(DataReaderImplGetTopicDescription(reader));` ��ȡ�������������⡣
 *
 * @param   NAME    Ϊ�ص���������ǰ׺��
 * @param   TYPE    ����ָ��������������͡�
 *
 * @details ���û���Ҫ�������ݻص��ӿ�ʱ������ȡ��ǰ׺ΪTrack���������������Ϊ DDS_ZeroCopyBytes ���ͣ�ͨ�����´�����ɣ�
 *
 *          @code{.c}
 *          // ����ʵ�����ݴ����������� Track �� DDS_ZeroCopyBytes Ϊ DDS_SimpleDataReaderListener �����
 *          // (DDS_DataReader* reader, DDS_ZeroCopyBytes* sample, DDS_SampleInfo* info)Ϊ Track_on_process_sample �ĺ�������
 *          // reader ��ʾ���ݵ�������ݶ���
 *          // sample ��ʾ�µ������������
 *          // info ��ʾ����������������������Ϣ
 *          SimpleDataReaderListener(Track, DDS_ZeroCopyBytes)(DDS_DataReader* reader, DDS_ZeroCopyBytes* sample, DDS_SampleInfo* info)
 *          {
 *              
 *          }
 *          // ��������
 *          // ...
 *          // �ڴ������ݶ���ʱ��Ϊ���ݶ��߼�������Ա��ֵ��
 *          DDS_DataReaderListener drListener;
 *          DDS_DataReaderListener_initial(&drListener);
 *          drListener.on_data_available = Track_on_data_available;
 *          @endcode
 */

#define DDS_SimpleDataReaderListener(NAME, TYPE)                                                \
    void NAME##_on_process_sample(DDS_DataReader* reader, TYPE* sample, DDS_SampleInfo* info);  \
    void NAME##_on_data_available(DDS_DataReader* the_reader)                                   \
    {                                                                                           \
        TYPE##Seq data_values;                                                                  \
        TYPE##Seq_initialize(&data_values);                                                     \
        DDS_SampleInfoSeq sample_infos;                                                         \
        DDS_SampleInfoSeq_initialize(&sample_infos);                                            \
        TYPE##DataReader* reader = (TYPE##DataReader*)the_reader;                               \
        DDS_ReturnCode_t retCode = TYPE##DataReader_take(reader, &data_values,                  \
            &sample_infos, LENGTH_UNLIMITED,                                                    \
            DDS_ANY_SAMPLE_STATE, DDS_ANY_VIEW_STATE, DDS_ANY_INSTANCE_STATE);                  \
        if (DDS_RETCODE_OK != retCode)                                                          \
        {                                                                                       \
            printf("take failed(%d.\n", retCode);                                               \
            return;                                                                             \
        }                                                                                       \
        ZR_UINT32 i;                                                                            \
        for (i = 0; i < sample_infos._length; i++)                                              \
        {                                                                                       \
            DDS_SampleInfo* info = DDS_SampleInfoSeq_get_reference(&sample_infos, i);           \
            if (!info->valid_data)                                                              \
            {                                                                                   \
                continue;                                                                       \
            }                                                                                   \
            TYPE* sample = TYPE##Seq_get_reference(&data_values, i);                 \
            NAME##_on_process_sample(the_reader, sample, info);                                 \
        }                                                                                       \
        TYPE##DataReader_return_loan(reader, &data_values, &sample_infos);                      \
        return;                                                                                 \
    }                                                                                           \
    void NAME##_on_process_sample

/**
 * @def DDS_DataReaderListener_initial(listener)
 *
 * @ingroup CListener
 *
 * @brief   ��ʼ�����ݶ��߼���������Ϊ���г�ԱΪ�ա�
 *
 * @param   listener    ��Ҫ��ʼ�������ݶ��߼���������
 */

#define DDS_DataReaderListener_initial(listener) memset(listener, 0, sizeof(DDS_DataReaderListener))

#endif /* DataReaderListener_h__*/
