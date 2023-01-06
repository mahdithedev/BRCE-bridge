use crate::error;

pub fn parse_header(buffer: &[u8]) -> Result<(u8 , u16) , error::ApplicationError> {

    if buffer.len() < 3 {
        return Err(error::ApplicationError(error::ApErCode::BYTESNOTENOUGH));
    }

    let packet_type = buffer[0];
    let payload_size = &buffer[1..3];

    let payload_size: u16 = (payload_size[0] | payload_size[1]<<7) as u16;

    Ok((packet_type , payload_size))
    
}

#[cfg(test)]
mod tests {

    use super::*;

    

}