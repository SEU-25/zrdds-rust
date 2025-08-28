/**
 * @file:       TopicListener.h
 *
 * copyright:   Copyright (c) 2018 ZhenRong Technology, Inc. All rights reserved.
 */

#ifndef TopicListener_h__
#define TopicListener_h__

#include "Listener.h"
#include "InconsistentTopicStatus.h"
#include "ZRDDSCWrapper.h"

/**
 * @typedef void(*TopicListenerInconsistentTopicCallback)( DDS_Topic* topic, const DDS_InconsistentTopicStatus* status)
 *
 * @ingroup CListener
 *
 * @brief   �ײ� #DDS_INCONSISTENT_TOPIC_STATUS ״̬�Ļص���������ָ�����͡�
 *
 * @param [in,out]  topic   �ü��������������⡣
 * @param   status          ��ǰ��״ֵ̬��
 */

typedef void(*TopicListenerInconsistentTopicCallback)(
    DDS_Topic* topic,
    const DDS_InconsistentTopicStatus* status);

/**
 * @struct DDS_TopicListener
 *
 * @ingroup CListener
 *
 * @brief   ������������͡�
 */

typedef struct DDS_TopicListener
{
    /** @brief   �����ࡱ��Ա�� */
    DDS_Listener listener;
    /** @brief  Ϊ #DDS_INCONSISTENT_TOPIC_STATUS ״̬���õĻص������� */
    TopicListenerInconsistentTopicCallback on_inconsistent_topic;
}DDS_TopicListener;

/**
 * @def DDS_TopicListener_initial(listener)
 *
 * @ingroup CListener
 *
 * @brief   ��ʼ���������������Ϊ���г�ԱΪ�ա�
 *
 * @param   listener    ��Ҫ��ʼ�����������������
 */

#define DDS_TopicListener_initial(listener) memset(listener, 0, sizeof(DDS_TopicListener))

#endif /* TopicListener_h__*/
