import React from 'react';
import { emit } from '@tauri-apps/api/event';

import './Form.css';

interface InputSlotProperties {
    type: string;
    name: string;
    required?: boolean;
    pattern?: string;
    icon?: string;
}

class InputSlot extends React.Component<InputSlotProperties> {
    render(): React.ReactNode {
        const { type, name, required, pattern } = this.props;
        return (
            <div className='form-field'>
                {this.props.type !== 'submit' ? (
                    <i className='material-icons icons'>{this.props.icon}</i>
                ) : (
                    ''
                )}
                {this.props.type === 'submit' ? (
                    <button
                        className='btn'
                        type='submit'
                        onClick={() => {
                            const data = new FormData(
                                document.querySelector(
                                    'form'
                                ) as HTMLFormElement
                            );
                            emit(
                                'start-task',
                                JSON.stringify(
                                    Object.fromEntries(data.entries())
                                )
                            );
                        }}
                    >
                        {name}
                    </button>
                ) : (
                    <input
                        type={type}
                        name={name}
                        placeholder={name}
                        required={required}
                        pattern={pattern}
                    />
                )}
            </div>
        );
    }
}

export class Form extends React.Component {
    render(): React.ReactNode {
        return (
            <form>
                <InputSlot
                    type='text'
                    name='学号'
                    required={true}
                    pattern='\d{8,10}'
                    icon='account_circle'
                />
                <InputSlot
                    type='password'
                    name='密码'
                    required={true}
                    icon='lock'
                />
                <InputSlot type='submit' name='开始自动打卡' />
            </form>
        );
    }
}
