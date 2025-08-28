/**
 * @file:       SubscriberListener.h
 *
 * copyright:   Copyright (c) 2018 ZhenRong Technology, Inc. All rights reserved.
 */

#ifndef SubscriberListener_h__
#define SubscriberListener_h__

#include "DataReaderListener.h"
#include "ZRDDSCWrapper.h"

/**
 * @typedef void(*SubscriberListenerDataOnReadersCallback)(DDS_Subscriber* sub)
 *
 * @ingroup  CListener
 *
 * @brief   �ص� #DDS_DATA_ON_READERS_STATUS ״̬�Ļص���������ָ�����͡�
 *
 * @details �������ߵ���ʵ�����ݶ����������ݴﵽʱ�������߽����ڴ�״̬����������������״̬��
 *          - �����߼��������õĹ��ĵ�״̬���� #DDS_DATA_ON_READERS_STATUS ��
 *          - �û����� #DDS_Subscriber_notify_datareaders ������
 *          - �û��ɹ����� DDS_SubscriberListener::on_data_available ������
 *          - �û����ø�����������������ݶ��ߵ� @ref read-take ϵ�з�����
 *
 * @param   sub �ü����������Ķ����ߡ�
 */

typedef void(*SubscriberListenerDataOnReadersCallback)(DDS_Subscriber* sub);

/**
 * @struct DDS_SubscriberListener
 *
 * @ingroup CListener
 *
 * @brief   �����߼��������͡�
 */

typedef struct DDS_SubscriberListener
{
    /** @brief   �ó�Ա���ڼ��������ߵ����ݶ�����ʵ���״̬�� */
    DDS_DataReaderListener datareader_listener;
    /** @brief  Ϊ #DDS_DATA_ON_READERS_STATUS ״̬���õĻص������� */
    SubscriberListenerDataOnReadersCallback on_data_on_readers;
}DDS_SubscriberListener;

/**
 * @def DDS_SubscriberListener_initial(listener)
 *
 * @ingroup CListener
 *
 * @brief   ��ʼ�������߼���������Ϊ���г�ԱΪ�ա�
 *
 * @param   listener    ��Ҫ��ʼ���Ķ����߼���������
 */

#define DDS_SubscriberListener_initial(listener) memset(listener, 0, sizeof(DDS_SubscriberListener))

#endif /* SubscriberListener_h__*/
